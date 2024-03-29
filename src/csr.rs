use std::fmt::Debug;
use std::sync::{Arc,RwLock};
use std::hash::Hash;
use std::collections::HashMap;
use crate::graph::{GraphErr,EdgeChange,Graph};

#[derive(Debug)]
pub struct CoarseCSR {
    offsets: Vec<usize>,
    edges: Vec<(usize, f64)>,
}

impl CoarseCSR {
    fn get_offsets(&self, id: usize) -> (usize, usize) {
        let (v, e) = self.get_size();
        
        (
            self.offsets[id],
            if id < v - 1 {
                self.offsets[id + 1]
            } else {
                e
            }
        )
    }

    // assumes that the nodes exist--i love helper functions!
    fn intern_update_or_add_edge(&mut self, from: usize, to: usize, weight: f64, can_update: bool, can_add: bool) -> Result<EdgeChange, GraphErr> {
        let (v, e) = self.get_size();
        
        let (off_start, off_end) = self.get_offsets(from);
        let new_edge = (to, weight);

        for i in off_start..=off_end {
            if i == off_end {
                if can_add {
                    self.edges.push(new_edge);

                    for j in (from + 1)..v {
                        self.offsets[j] += 1;
                    }
                    return Ok(EdgeChange::Added);
                } else {
                    return Err(GraphErr::NoSuchEdge);
                }
            }

            if self.edges[i].0 == to {
                if can_update {
                    let old_w = self.edges[i].1;
                    self.edges[i].1 = weight;
                    return Ok(EdgeChange::Updated(old_w));
                } else {
                    return Err(GraphErr::EdgeAlreadyExists);
                }
            } else if self.edges[i].0 > to { // need to insert at this index
                if can_add {
                    self.edges.insert(i, new_edge);
                    
                    for j in (from + 1)..v {
                        self.offsets[j] += 1;
                    }
                    return Ok(EdgeChange::Added);
                } else {
                    return Err(GraphErr::NoSuchEdge);                    
                }
            }
        }

        unreachable!()
    }
}

impl CoarseCSR {
    fn new() -> Self {
        Self {
            offsets: vec!(),
            edges: vec!()
        }
    }
    
    fn get_size(&self) -> (usize, usize) {
        (
            self.offsets.len(),
            self.edges.len()
        )
    }

    fn get_edge(&self, from: usize, to: usize) -> Result<f64, GraphErr> {
        let (_, e) = self.get_size();
        
        if from >= self.offsets.len() || to >= self.offsets.len() {
            Err(GraphErr::NoSuchNode)
        } else {
            let (off_start, off_end) = self.get_offsets(from);

            // TODO binary search for better time complexity :/
            for i in off_start..off_end {
                if i >= e {
                    break;
                }
                
                if self.edges[i].0 == to {
                    return Ok(self.edges[i].1);
                }
            }

            Err(GraphErr::NoSuchEdge)
        }
    }

    fn get_nodes(&self) -> Vec<usize> {
        (0..self.offsets.len()).collect()
    }
    
    fn get_edges(&self) -> Vec<(usize, usize, f64)> {
        let mut ret = vec!();
        let (v, e) = self.get_size();
        for i in 0..v {
            let (off_start, off_end) = self.get_offsets(i);
            for j in off_start..off_end {
                let edge = self.edges[j];
                ret.push((i, edge.0, edge.1));
            }
        }
        
        ret
    }
    
    fn get_neighbors(&self, id: usize) -> Result<Vec<usize>, GraphErr> {
        Err(GraphErr::NoSuchNode)
    }

    fn get_node_label(&self, id: usize) -> Result<f64, GraphErr> {
        Err(GraphErr::NoSuchNode)
    }

    fn set_node_label(&self, id: usize, label: f64) -> Result<f64, GraphErr> {
        Err(GraphErr::NoSuchNode)
    }
    
    fn add_edge(&mut self, from: usize, to: usize, weight: f64) -> Result<(), GraphErr> {
        match self.intern_update_or_add_edge(from, to, weight, false, true) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }

    fn update_edge(&mut self, from: usize, to: usize, weight: f64) -> Result<f64, GraphErr> {
        match self.intern_update_or_add_edge(from, to, weight, false, true) {
            Ok(EdgeChange::Updated(w)) => Ok(w),
            Err(e) => Err(e),
            _ => unreachable!()
        }
    }

    fn update_or_add_edge(&mut self, from: usize, to: usize, weight: f64) -> Result<EdgeChange, GraphErr> {
        self.intern_update_or_add_edge(from, to, weight, true, true)
    }

    fn add_node(&mut self, id: usize) -> Result<(), GraphErr> {
        let (v, e) = self.get_size();
            
        if id < v {
            Err(GraphErr::NodeAlreadyExists)
        } else if id > v {
            panic!("invalid node id {} passed to csr add_node\n", id);
        } else {
            self.offsets.push(e);
            Ok(())
        }
    }
    
    fn debug(&self) {
        println!("{:?}", self);
    }

    fn remove_edge(&mut self, from: usize, to: usize) -> Result<f64, GraphErr> {
        Err(GraphErr::NoSuchEdge)
    }

    fn remove_node(&mut self, id: usize) -> Result<(), GraphErr> {
        Err(GraphErr::NoSuchNode)
    }
}

/*
impl CoarseCSR {
    fn remove_edge(&mut self, from: usize, to: usize) -> Result<f64, GraphErr> {
        match self.get_edge(from, to) {
            Ok(old_weight) => {
                let (v, _) = self.get_size();
                let (off_start, off_end) = self.get_offsets(from);
                
                // insert edge into edge list
                // TODO this could be a more effective data structure 🤪 don't use linear search?
                for i in off_start..off_end {
                    if self.edges[i].0 == to { // need to insert at this index
                        self.edges.remove(i);
                        break;
                    }
                }
                
                // shift offsets
                for i in (from + 1)..v {
                    self.offsets[i] -= 1;
                }

                Ok(old_weight)
            },
            Err(e) => Err(e)
        }
    }

    fn remove_node(&mut self, id: usize) -> Result<(), GraphErr> {
        let (v, _) = self.get_size();

        if id >= v {
            Err(GraphErr::NoSuchNode)
        } else {
            // remove the node's edges
            let (off_start, off_end) = self.get_offsets(id);
            self.edges.drain(off_start..off_end);

            // remove the node from the offset table
            self.offsets.remove(id);

            // shift everyone else's offsets
            let shift = off_end - off_start;
            for i in id..(v - 1) {
                self.offsets[i] -= shift
            }

            // update EVERYTHING augh
            let (v, e) = self.get_size();
            let mut new_offsets: Vec<usize> = vec!();
            let mut new_edges: Vec<usize> = vec!();
            for i in 0..e {
                // TODO
            }
            
            Ok(())
        }
    }
}
*/

pub struct CoarseCSRGraph<Id: Clone + Debug + Eq + Hash> {
    csr: Arc<RwLock<CoarseCSR>>,
    internal_ids: Arc<RwLock<HashMap<Id, usize>>>,
}

impl<Id: Clone + Debug + Eq + Hash> CoarseCSRGraph<Id> {
    fn get_id(&self, id_: &Id) -> Result<usize, GraphErr> {
        let map = self.internal_ids.read().unwrap();
        
        match map.get(id_) {
            Some(id) => Ok(id.clone()),
            _ => Err(GraphErr::NoSuchNode),
        }
    }

    fn get_ids(&self, id0_: &Id, id1_: &Id) -> Result<(usize, usize), GraphErr> {
        let map = self.internal_ids.read().unwrap();
        
        match (map.get(id0_), map.get(id1_)) {
            (Some(id0), Some(id1)) => Ok((id0.clone(), id1.clone())),
            _ => {
                Err(GraphErr::NoSuchNode)
            }
        }
    }
}

impl<Id: Clone + Debug + Eq + Hash> Graph<Id> for CoarseCSRGraph<Id> {
    fn new() -> Self {
        Self {
            csr: Arc::new(RwLock::new(CoarseCSR::new())),
            internal_ids: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    fn get_size(&self) -> (usize, usize) {
        let csr = self.csr.read().unwrap();
        csr.get_size()
    }

    fn get_edge(&self, from: Id, to: Id) -> Result<f64, GraphErr> {
        match self.get_ids(&from, &to) {
            Ok((f_, t_)) => {
                let csr = self.csr.read().unwrap();
                csr.get_edge(f_, t_)
            },
            Err(e) => Err(e)
        }
    }

    fn get_nodes(&self) -> Vec<Id> {
        vec!()
    }
    
    fn get_edges(&self) -> Vec<(Id, Id, f64)> {
        /*
        let csr = self.csr.read().unwrap();
        let map = self.internal_ids.read().unwrap();
        let csr_edges = csr.get_edges();
        csr_edges
            .into_iter()
            .map(
                |(from_, to_, weight)| {
                    (map.get(from_).unwrap().clone(), map.get(to_).unwrap().clone(), weight)
                }
            )
            .collect()
        */
        vec!()
    }

    fn get_neighbors(&self, id: Id) -> Result<Vec<Id>, GraphErr> {
        Err(GraphErr::NoSuchNode)
    }

    fn get_node_label(&self, id: Id) -> Result<f64, GraphErr> {
        Err(GraphErr::NoSuchNode)
    }

    fn set_node_label(&self, id: Id, label: f64) -> Result<f64, GraphErr> {
        Err(GraphErr::NoSuchNode)
    }
    
    fn add_edge(&self, from: Id, to: Id, weight: f64) -> Result<(), GraphErr> {
        match self.get_ids(&from, &to) {
            Ok((f_, t_)) => {
                let mut csr = self.csr.write().unwrap();
                csr.add_edge(f_, t_, weight)
            },
            Err(e) => Err(e)
        }
    }

    fn update_edge(&self, from: Id, to: Id, weight: f64) -> Result<f64, GraphErr> {
        match self.get_ids(&from, &to) {
            Ok((f_, t_)) => {
                let mut csr = self.csr.write().unwrap();
                csr.update_edge(f_, t_, weight)
            },
            Err(e) => Err(e)
        }
    }

    fn update_or_add_edge(&self, from: Id, to: Id, weight: f64) -> Result<EdgeChange, GraphErr> {
        match self.get_ids(&from, &to) {
            Ok((f_, t_)) => {
                let mut csr = self.csr.write().unwrap();
                csr.update_or_add_edge(f_, t_, weight)
            },
            Err(e) => Err(e)
        }
    }

    fn add_node(&self, id: Id) -> Result<(), GraphErr> {
        match self.get_id(&id) {
            Ok(_) => {
                Err(GraphErr::NodeAlreadyExists)
            },
            _ => {
                let mut ids = self.internal_ids.write().unwrap();
                let mut csr = self.csr.write().unwrap();

                let (v, _) = csr.get_size();
                
                ids.insert(id, v);
                csr.add_node(v)
            }
        }
    }

    fn debug(&self) {
        let map = self.internal_ids.read().unwrap();
        println!("map:");
        for (k, v) in map.iter() {
            println!("{:?} -> {:?}", k, v);
        }

        let csr = self.csr.read().unwrap();
        println!("csr:");
        println!("{:?}", csr);
    }

    fn remove_edge(&self, from: Id, to: Id) -> Result<f64, GraphErr> {
        Err(GraphErr::NoSuchEdge)
    }

    fn remove_node(&self, id: Id) -> Result<(), GraphErr> {
        Err(GraphErr::NoSuchNode)
    }
}

/*
impl<Id: Clone + Debug + Eq + Hash> CoarseCSRGraph<Id> {
    fn remove_edge(&mut self, from: Id, to: Id) -> Result<f64, GraphErr> {
        match self.get_ids(&from, &to) {
            Ok((f_, t_)) => {
                let mut csr = self.csr.write().unwrap();
                csr.remove_edge(f_, t_)
            },
            Err(e) => Err(e)
        }
    }
    
    fn remove_node(&mut self, id: Id) -> Result<(), GraphErr> {
        match self.get_id(&id) {
            Ok(mut id_) => {
                let mut ids = self.internal_ids.write().unwrap();
                let mut csr = self.csr.write().unwrap();

                ids.remove(&id);
                for (_, v) in ids.iter_mut() {
                    if v > &mut id_ {
                        *v = *v - 1;
                    }
                }
                
                csr.remove_node(id_)
            },
            _ => {
                Err(GraphErr::NodeAlreadyExists)
            }
        }
    }
}
*/

unsafe impl<Id: Clone + Debug + Eq + Hash> Send for CoarseCSRGraph<Id> {}
unsafe impl<Id: Clone + Debug + Eq + Hash> Sync for CoarseCSRGraph<Id> {}

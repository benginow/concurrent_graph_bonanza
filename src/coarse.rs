use std::sync::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use std::hash::Hash;
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
    fn internal_update_or_add_edge(&mut self, from: usize, to: usize, weight: f64, is_update: bool) {
        let (v, e) = self.get_size();
        
        if !is_update {
            // add edge
            let (off_start, off_end) = self.get_offsets(from);
            let new_edge = (to, weight);

            // insert edge into edge list
            // TODO this could be a more effective data structure 🤪 don't use linear search?
            for i in off_start..off_end {
                if i == e {
                    self.edges.push(new_edge);
                    break;
                }

                if self.edges[i].0 > to { // need to insert at this index
                    self.edges.insert(i, new_edge);
                    break;
                }
            }

            // shift offsets
            for i in (from + 1)..v {
                self.offsets[i] += 1;
            }
        } else {
            let (off_start, off_end) = self.get_offsets(from);

            for i in off_start..off_end {
                if self.edges[i].0 == to {
                    self.edges[i].1 = weight;
                    break;
                }
            }
        }
    }
}

impl Graph<usize> for CoarseCSR {
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

    fn get_neighbors(&self, id: usize) -> Result<Vec<usize>, GraphErr> {
        Err(GraphErr::NoSuchNode)
    }

    fn add_edge(&mut self, from: usize, to: usize, weight: f64) -> Result<(), GraphErr> {
        match self.get_edge(from, to) {
            Ok(_) => Err(GraphErr::EdgeAlreadyExists),
            Err(e) => match e {
                GraphErr::NoSuchNode => Err(e),
                _ => {
                    self.internal_update_or_add_edge(from, to, weight, false);
                    Ok(())
                }
            }
        }
    }

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

    fn update_edge(&mut self, from: usize, to: usize, weight: f64) -> Result<f64, GraphErr> {
        match self.get_edge(from, to) {
            Ok(old_weight) => {
                self.internal_update_or_add_edge(from, to, weight, true);
                Ok(old_weight)
            },
            Err(e) => Err(e)
        }
    }

    fn update_or_add_edge(&mut self, from: usize, to: usize, weight: f64) -> Result<EdgeChange, GraphErr> {
        match self.get_edge(from, to) {
            Ok(old_weight) => {
                self.internal_update_or_add_edge(from, to, weight, true);
                Ok(EdgeChange::Updated(old_weight))
            },
            Err(e) => match e {
                GraphErr::NoSuchEdge => {
                    self.internal_update_or_add_edge(from, to, weight, true);
                    Ok(EdgeChange::Added)
                },
                _ => Err(e),
            }
        }
    }

    fn add_node(&mut self, id: usize) -> Result<(), GraphErr> {
        let (v, _) = self.get_size();
            
        if id < v {
            Err(GraphErr::NodeAlreadyExists)
        } else if id > v {
            panic!("invalid node id {} passed to csr add_node\n", id);
        } else {
            self.offsets.push(self.edges.len());
            Ok(())
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
            
            Ok(())
        }
    }
}

struct CoarseCSRGraph<Id: Clone + Eq + Hash> {
    csr: Arc<RwLock<CoarseCSR>>,
    internal_ids: Arc<RwLock<HashMap<Id, usize>>>,
}

impl<Id: Clone + Eq + Hash> CoarseCSRGraph<Id> {
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

impl<Id: Clone + Eq + Hash> Graph<Id> for CoarseCSRGraph<Id> {
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

    fn get_neighbors(&self, id: Id) -> Result<Vec<Id>, GraphErr> {
        Err(GraphErr::NoSuchNode)
    }

    fn add_edge(&mut self, from: Id, to: Id, weight: f64) -> Result<(), GraphErr> {
        match self.get_ids(&from, &to) {
            Ok((f_, t_)) => {
                let mut csr = self.csr.write().unwrap();
                csr.add_edge(f_, t_, weight)
            },
            Err(e) => Err(e)
        }
    }

    fn remove_edge(&mut self, from: Id, to: Id) -> Result<f64, GraphErr> {
        match self.get_ids(&from, &to) {
            Ok((f_, t_)) => {
                let mut csr = self.csr.write().unwrap();
                csr.remove_edge(f_, t_)
            },
            Err(e) => Err(e)
        }
    }

    fn update_edge(&mut self, from: Id, to: Id, weight: f64) -> Result<f64, GraphErr> {
        match self.get_ids(&from, &to) {
            Ok((f_, t_)) => {
                let mut csr = self.csr.write().unwrap();
                csr.update_edge(f_, t_, weight)
            },
            Err(e) => Err(e)
        }
    }

    fn update_or_add_edge(&mut self, from: Id, to: Id, weight: f64) -> Result<EdgeChange, GraphErr> {
        match self.get_ids(&from, &to) {
            Ok((f_, t_)) => {
                let mut csr = self.csr.write().unwrap();
                csr.update_or_add_edge(f_, t_, weight)
            },
            Err(e) => Err(e)
        }
    }

    fn add_node(&mut self, id: Id) -> Result<(), GraphErr> {
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

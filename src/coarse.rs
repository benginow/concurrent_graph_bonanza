use std::sync::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use std::hash::Hash;
use crate::graph::{GraphErr,EdgeChange,Graph};

struct CoarseCSR {
    offsets: Vec<u64>,
    edges: Vec<(u64, f64)>,
}

impl CoarseCSR {
    // guaranteed that the nodes exist
    fn internal_update_or_add_edge(&mut self, from: u64, to: u64, weight: f64, is_update: bool) {

    }
}

impl Graph<u64> for CoarseCSR {
    fn new() -> Self {
        // TODO
    }
    
    fn get_size(&self) -> (u64, u64) {
        (
            self.offsets.len(),
            self.edges.len()
        )
    }

    fn get_edge(&self, from: u64, to: u64) -> Result<f64, GraphErr> {
        let (v, e) = (self.offsets.len(), self.edges.len());
        
        if from >= self.offsets.len() || to >= self.offsets.len() {
            Err(GraphErr::NoSuchNode)
        } else {
            let off_start = offset[from];
            let off_end =
                if from < v - 1 {
                    offset[from + 1]
                } else {
                    e
                };

            // TODO binary search for better time complexity :/
            for i in off_start..off_end {
                if self.edges[i].0 == to {
                    return Ok(self.edges[i].2);
                }
            }

            Err(GraphErr::NoSuchEdge)
        }
    }

    fn add_edge(&mut self, from: u64, to: u64, weight: f64) -> Result<(), GraphErr> {
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

    fn remove_edge(&mut self, from: u64, to: u64, weight: f64) -> Result<f64, GraphErr> {
        match self.get_edge(from, to) {
            Ok(_) => {
                // TODO AAAAAAAA
            },
            Err(e) => Err(e)
        }
    }

    fn update_edge(&mut self, from: u64, to: u64, weight: f64) -> Result<f64, GraphErr> {
        match self.get_edge(from, to) {
            Ok(old_weight) => {
                self.internal_update_or_add_edge(from, to, weight, true);
                Ok(old_weight)
            },
            Err(e) => Err(e)
        }
    }

    fn update_or_add_edge(&mut self, from: u64, to: u64, weight: f64) -> Result<EdgeChange, GraphErr> {
        match self.get_edge(from, to) {
            Ok(old_weight) => {
                self.internal_update_or_add_edge(from, to, weight, true);
                Ok(EdgeChange::Updated(old_weight))
            },
            Err(e) => {
                GraphErr::NoSuchEdge => {
                    self.internal_update_or_add_edge(from, to, weight, true);
                    Ok(EdgeChange::Added)
                },
                _ => Err(e),
            }
        }
    }
}

struct CoarseGraph<Id: Hash + Eq> {
    csr: Arc<RwLock<CoarseCSR>>,
    user_to_internal_ids: Arc<RwLock<HashMap<Id, u64>>>,
}

impl<Id: Hash + Eq> Graph<Id> for CoarseGraph<Id> {
    fn new() -> Self {
        Self {
            // TODO
        }
    }
    
    fn get_size(&self) -> (u64, u64) {
        let csr = self.csr.read().unwrap();
        csr.get_size()
    }

    fn get_edge(&self, from: Id, to: Id) -> Result<f64, GraphErr> {
        let map = self.user_to_internal_ids.read().unwrap();
        let (from_, to_) = match (map.get(from), map.get(to)) {
            (Some(f), Some(t)) => (f, t),
            _ => {
                return GraphErr::NoSuchNode;
            }
        };
        map.drop(); // TODO does this drop the map or the lockguard?
        
        let csr = self.csr.read().unwrap();
        csr.get_edge(from_, to_)
    }

    fn add_edge(&mut self, from: Id, to: Id, weight: f64) -> Result<(), GraphErr> {
        Err(GraphErr::NoSuchNode)
    }

    fn remove_edge(&mut self, from: Id, to: Id) -> Result<f64, GraphErr> {
        Err(GraphErr::NoSuchEdge)
    }

    fn update_edge(&mut self, from: Id, to: Id, weight: f64) -> Result<f64, GraphErr> {
        Err(GraphErr::NoSuchEdge)
    }

    fn update_or_add_edge(&mut self, from: Id, to: Id, weight: f64) -> Result<f64, GraphErr> {
        Err(GraphErr::NoSuchNode)
    }

    fn add_node(&mut self, id: Id) -> Result<(), GraphErr> {
        Err(GraphErr::NodeAlreadyExists)
    }

    fn remove_node(&mut self, id: Id) -> Result<(), GraphErr> {
        Err(GraphErr::NoSuchNode)
    }
}

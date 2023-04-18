use std::sync::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use std::hash::Hash;
use crate::graph::{EdgeChange,GraphErr,Graph};


pub struct SimpleGraph<Id: Clone + Eq + Hash> {

    vertex_counter: usize,
    edge_counter: usize,
    // map from user's id to our id
    map: RwLock<HashMap<Id, usize>>,
    // index and weight
    graph: Arc<RwLock<Vec<Arc<RwLock<Vec<(usize, f64)>>>>>>,
    // parents: RwLock<HashMap<Id,usize>>
}


impl<Id: Clone + Eq + Hash> Graph<Id> for SimpleGraph<Id> {
    fn new() -> Self {
        Self { 
            vertex_counter: 0,
            edge_counter: 0,
            map: RwLock::new(HashMap::new()),
            graph: Arc::new(RwLock::new(vec![])),
            // this is a list of all parent nodes
        }
    }

    fn get_size(&self) -> (usize, usize) {
        return (self.vertex_counter, self.edge_counter);
    }

    fn get_edge(&self, from: Id, to: Id) -> Result<f64, GraphErr> {
        let read_map = self.map.read().unwrap();
       
        // get ids first
        let from_id = read_map.get(&from);
        match from_id {
            Some(from_id) => {
                let to_id = read_map.get(&to);
                match to_id {
                    Some(to) => {
                        // now, add to the graph if and only if that edge doesnt already exist in the map
                        let read_g = self.graph.read().unwrap();
                        let read_graph = read_g[*from_id].read().unwrap();
                        let val = read_graph.iter().find(|(id, _)| id == to);
                        
                        match val {
                            Some(v) => {
                                Ok(v.1)
                            }
                            None => {
                                Err(GraphErr::NoSuchNode)
                            }
                        }
                    }
                    None => {
                        Err(GraphErr::NoSuchNode)
                    }
                }
            }
            None => {
                Err(GraphErr::NoSuchNode)
            }
        }
    }

    fn get_neighbors(&self, id: Id) -> Result<Vec<Id>, GraphErr> {
        Err(GraphErr::NoSuchNode)
    }
    
    fn add_edge(&mut self, from: Id, to: Id, weight: f64) -> Result<(), GraphErr> {
        // first, transform to ids
        let read_map = self.map.read().unwrap();

       
        // get ids first
        let from_id = read_map.get(&from);
        match from_id {
            Some(from_id) => {
                let to_id = read_map.get(&to);
                match to_id {
                    Some(to) => {
                        // now, add to the graph if and only if that edge doesnt already exist in the map
                        if self.graph.read().unwrap()[*from_id].read().unwrap().iter().find(|(id, _)| id == to).is_some() {
                            return Err(GraphErr::EdgeAlreadyExists)
                        }
                        
                        self.graph.read().unwrap()[*from_id].write().unwrap().push((*to, weight));
                        Ok(())
                    }
                    None => {
                        Err(GraphErr::NoSuchNode)
                    }
                }
            }
            None => {
                Err(GraphErr::NoSuchNode)
            }
        }
    }

    fn remove_edge(&mut self, from: Id, to: Id) -> Result<f64, GraphErr> {
        
        let read_map = self.map.read().unwrap();
        // get ids first
        let from_id = read_map.get(&from);
        match from_id {
            Some(from_id) => {
                let to_id = read_map.get(&to);
                match to_id {
                    Some(to) => {
                        // now, add to the graph if and only if that edge doesnt already exist in the map
                        let read_g = self.graph.read().unwrap();
                        let read_graph = read_g[*from_id].read().unwrap();
                        let idx = read_graph.iter().enumerate().find(|(id, _)| id == to);
                        match idx {
                            Some (x) => {
                                self.graph.read().unwrap()[*from_id].write().unwrap().remove(x.0);
                                Ok(x.1.1)
                            }
                            None => {
                                Err(GraphErr::NoSuchEdge)
                            }
                        }
                    }
                    None => {
                        Err(GraphErr::NoSuchNode)
                    }
                }
            }
            None => {
                Err(GraphErr::NoSuchNode)
            }
        }
    }

    fn update_edge(&mut self, from: Id, to: Id, weight: f64) -> Result<f64, GraphErr> {
        let read_map = self.map.read().unwrap();
        // get ids first
        let from_id = read_map.get(&from);
        match from_id {
            Some(from_id) => {
                let to_id = read_map.get(&to);
                match to_id {
                    Some(to) => {
                        // now, add to the graph if and only if that edge doesnt already exist in the map
                        let read_g = self.graph.read().unwrap();
                        let read_graph = read_g[*from_id].read().unwrap();
                        let idx = read_graph.iter().enumerate().find(|(id, _)| id == to);
                        match idx {
                            Some (x) => {
                                self.graph.read().unwrap()[*from_id].write().unwrap()[x.0] = (*to, weight);
                                return Ok(x.1.1);
                            }
                            None => {
                                return Err(GraphErr::NoSuchEdge);
                            }
                        }
                    }
                    None => {
                        return Err(GraphErr::NoSuchNode);
                    }
                }
            }
            None => {
                return Err(GraphErr::NoSuchNode);
            }
        }
    }

    fn update_or_add_edge(&mut self, from: Id, to: Id, weight: f64) -> Result<EdgeChange, GraphErr> {
        let read_map = self.map.read().unwrap();
        // get ids first
        let from_id = read_map.get(&from);
        match from_id {
            Some(from_id) => {
                let to_id = read_map.get(&to);
                match to_id {
                    Some(to) => {
                        // now, add to the graph if and only if that edge doesnt already exist in the map
                        let read_g = self.graph.read().unwrap();
                        let read_graph = read_g[*from_id].read().unwrap();
                        let idx = read_graph.iter().enumerate().find(|(id, _)| id == to);
                        match idx {
                            Some (x) => {
                                self.graph.read().unwrap()[*from_id].write().unwrap()[x.0] = (*to, weight);
                                return Ok(EdgeChange::Updated(x.1.1));
                            }
                            None => {
                                self.graph.read().unwrap()[*from_id].write().unwrap().push((*to, weight));
                                Ok(EdgeChange::Added)
                            }
                        }
                    }
                    None => {
                        return Err(GraphErr::NoSuchNode);
                    }
                }
            }
            None => {
                return Err(GraphErr::NoSuchNode);
            }
        }
    }

    fn add_node(&mut self, id: Id) -> Result<(), GraphErr> {
        let read_map = self.map.read().unwrap();
        let read_id = read_map.get(&id);

        match read_id {
            Some(_) => {
                Err(GraphErr::NodeAlreadyExists)
            }
            None => {
                self.graph.write().unwrap().push(Arc::new(RwLock::new(vec![])));
                Ok(())
            }
        }

    }

    // enough just to remove connections --  the id will now just be ignored, which is super space inefficient!
    // extremely slow -- sequential search 
    // messiest code of all time -- sorry!
    fn remove_node(&mut self, id: Id) -> Result<(), GraphErr> {
        let read_map = self.map.read().unwrap();
        let read_id = read_map.get(&id);

        match read_id {
            Some(read_id) => {
                let rg_placeholder = self.graph.read().unwrap();
                let read_graph = rg_placeholder.iter();
                // iterate over rows
                for r in read_graph {
                    // really awful performance, because we have to lock this every time
                    let mut remove_index = None;
                    let row_placeholder = r.read().unwrap();
                    let row = row_placeholder.iter();
                    
                    for (idx, val) in row.enumerate() {
                        if val.0 == *read_id {
                            remove_index = Some(idx);
                        }
                    }
                    match remove_index {
                        Some(i) => {
                            let mut write_row = r.write().unwrap();
                            write_row.remove(i);
                        }
                        None => ()
                    }
                }
                Ok(())
            }
            None => {
                Err(GraphErr::NoSuchEdge)
            }
        }
    }
}

use std::fmt::Debug;
use std::sync::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use std::hash::Hash;
use crate::graph::{EdgeChange,GraphErr,Graph};
use std::sync::atomic::{AtomicUsize, Ordering};

// things that can be done!
// change orderings and see how this affects timings



pub struct SimpleGraph<Id: Clone + Debug + Eq + Hash> {

    vertex_counter: AtomicUsize,
    edge_counter: AtomicUsize,
    // map from user's id to our id
    map: RwLock<HashMap<Id, usize>>,
    // need to update this every time a node is added!
    id_to_value: RwLock<HashMap<usize, Id>>,
    // index and weight
    graph: Arc<RwLock<Vec<Arc<RwLock<Vec<(usize, f64)>>>>>>,
    // labels -> id to 
    labels: RwLock<HashMap<Id, f64>>,
}

// unsafe impl Sync for SimpleGraph<Id: Clone + Debug + Eg + Hash + Sync> {}
// unsafe impl Send for SimpleGraph<Id: Clone + Debug + Eg + Hash> {}

impl<Id: Clone + Debug + Eq + Hash + Copy> Graph<Id> for SimpleGraph<Id> {
    fn new() -> Self {
        println!("making graph");
        Self { 
            vertex_counter: AtomicUsize::new(0),
            edge_counter: AtomicUsize::new(0),
            map: RwLock::new(HashMap::new()),
            id_to_value: RwLock::new(HashMap::new()),
            graph: Arc::new(RwLock::new(vec![])),
            labels: RwLock::new(HashMap::new()),
            // this is a list of all parent nodes
        }
    }

    // if node is not labeled, it returns 0
    fn set_node_label(&self, of: Id, lbl: f64) -> Result<f64, GraphErr> {
        let read_map = self.map.read().unwrap();
        let from_id = read_map.get(&of);
        match from_id {
            Some(id) => {
                let mut read_labels = self.labels.read().unwrap();
                let label = read_labels.get(&of).clone();
                let retval = match label {
                    Some(l) => return Ok(*l),
                    // no need to update until later when it matters
                    None => Ok(0.0),
                };
                drop(read_labels);
                let mut write_labels = self.labels.write().unwrap();
                write_labels.insert(of, lbl);
                drop(write_labels);
                return retval;
            }  
            None => {
                return Err(GraphErr::NoSuchNode);
            }
        }
    }

    fn get_node_label(&self, of: Id) -> Result<f64, GraphErr> {
        let read_map = self.map.read().unwrap();
        let from_id = read_map.get(&of);
        match from_id {
            Some(id) => {
                let read_labels = self.labels.read().unwrap();
                let label = read_labels.get(&of);
                match label {
                    Some(l) => return Ok(*l),
                    // no need to update until later when it matters
                    None => Ok(0.0),
                }

            }
            None => {
                return Err(GraphErr::NoSuchNode);
            }
        }

    }

    fn get_size(&self) -> (usize, usize) {
        return (self.vertex_counter.load(Ordering::SeqCst), self.edge_counter.load(Ordering::SeqCst));
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
        // get id
        let read_map = self.map.read().unwrap();
        let from_id = read_map.get(&id);

        match from_id {
            Some(id) => {
                let read_graph = self.graph.read().unwrap();
                // [id].read().unwrap();
                let row = read_graph[*id].read().unwrap();
                let copied = row.clone();
                let collected: Vec<usize> = copied.iter().map(|&(s, _)| s).collect();
                let ids: Vec<Id> = collected.iter().map(|&s| *self.id_to_value.read().unwrap().get(&s).unwrap()).collect();
                return Ok(ids);
            }
            None => {
                return Err(GraphErr::NoSuchNode);
            }
        }
    }
    
    fn add_edge(&self, from: Id, to: Id, weight: f64) -> Result<(), GraphErr> {
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
                        self.vertex_counter.fetch_add(1, Ordering::SeqCst);
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

    fn remove_edge(&self, from: Id, to: Id) -> Result<f64, GraphErr> {
        let read_map = self.map.read().unwrap();
        // get ids first
        let from_id = read_map.get(&from);
        match from_id {
            Some(from_id) => {
                let to_id = read_map.get(&to);
                match to_id {
                    Some(to) => {
                        // now, add to the graph if and only if that edge doesnt already exist in the map
                        let mut read_g = self.graph.read().unwrap();
                        let mut read_graph = read_g[*from_id].write().unwrap();
                        // let idx = read_graph.iter().enumerate().find(|(id, _)| id == to);
                        let idx = read_graph.iter().position(|(id, _)| id == to);

                        match idx {
                            Some (x) => {
                                let edge_info = read_graph[x];
                                read_graph.remove(x);
                                Ok(edge_info.1)
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


    fn update_edge(&self, from: Id, to: Id, weight: f64) -> Result<f64, GraphErr> {
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

    fn update_or_add_edge(&self, from: Id, to: Id, weight: f64) -> Result<EdgeChange, GraphErr> {
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
                        // need a way to take ids -> 
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

    fn add_node(&self, id: Id) -> Result<(), GraphErr> {
        let read_map = self.map.read().unwrap();
        let read_id = read_map.get(&id);
        
        print!("we have read the thing");

        match read_id {
            Some(_) => {
                return Err(GraphErr::NodeAlreadyExists);
            }
            None => ()
        }

        drop(read_map);

        let mut writing = self.graph.write().unwrap();
                writing.push(Arc::new(RwLock::new(vec![])));
                let length = writing.len();
                self.map.write().unwrap().insert(id, length);
                self.id_to_value.write().unwrap().insert(length, id);
                Ok(())

    }

    // enough just to remove connections --  the id will now just be ignored, which is super space inefficient!
    // extremely slow -- sequential search 
    // messiest code of all time -- sorry!
    fn remove_node(&self, id: Id) -> Result<(), GraphErr> {
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
                            remove_index = Some(idx.clone());
                        }
                    }
                    // drop(row);
                    drop(row_placeholder);
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

    fn debug(&self) {
        ()
    }
}

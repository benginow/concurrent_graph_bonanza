use std::fmt::Debug;
use std::sync::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use std::hash::Hash;
use crate::graph::{EdgeChange,GraphErr,Graph};
use std::sync::atomic::{AtomicUsize, Ordering};

// things that can be done!
// change orderings and see how this affects timings
pub struct CSimpleGraph<Id: Clone + Debug + Eq + Hash> {
    removed_vertex_counter: AtomicUsize,
    removed_edge_counter: AtomicUsize,
    vertex_counter: AtomicUsize,
    edge_counter: AtomicUsize,
    // map from user's id to our id
    map: RwLock<HashMap<Id, usize>>,
    // need to update this every time a node is added!
    id_to_value: RwLock<HashMap<usize, Id>>,
    // index and weight
    graph: Arc<RwLock<Vec<Vec<(usize, f64)>>>>,
    // labels -> id to 
    labels: RwLock<HashMap<Id, f64>>,
}

unsafe impl <Id: Clone + Debug + Eq + Hash> Sync for CSimpleGraph<Id> {}
unsafe impl <Id: Clone + Debug + Eq + Hash + Send> Send for CSimpleGraph<Id> {}

impl<Id: Clone + Debug + Eq + Hash + Copy> Graph<Id> for CSimpleGraph<Id> {
    fn new() -> Self {
        println!("making graph");
        Self { 
            vertex_counter: AtomicUsize::new(0),
            edge_counter: AtomicUsize::new(0),
            removed_vertex_counter: AtomicUsize::new(0),
            removed_edge_counter: AtomicUsize::new(0),
            map: RwLock::new(HashMap::new()),
            id_to_value: RwLock::new(HashMap::new()),
            graph: Arc::new(RwLock::new(vec![])),
            labels: RwLock::new(HashMap::new()),
        }
    }

    // if node is not labeled, it returns 0
    fn set_node_label(&self, of: Id, lbl: f64) -> Result<f64, GraphErr> {
        let read_map = self.map.read().unwrap();
        let from_id = read_map.get(&of);
        match from_id {
            Some(_id) => {
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
            Some(_id) => {
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
        // println!("\nohohoh{0}", self.removed_vertex_counter.load(Ordering::SeqCst));
        return (self.vertex_counter.load(Ordering::SeqCst) - self.removed_vertex_counter.load(Ordering::SeqCst), self.edge_counter.load(Ordering::SeqCst) - self.removed_edge_counter.load(Ordering::SeqCst))
    }

    // no-op for now
    fn get_edge(&self, from: Id, to: Id) -> Result<f64, GraphErr> {
        Err(GraphErr::NoSuchNode)
    }

    // no-po for now for the sake of benchmarking
    fn get_neighbors(&self, id: Id) -> Result<Vec<Id>, GraphErr> {
        Err(GraphErr::NoSuchNode)
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
                        let mut write_graph = self.graph.write().unwrap();
                        // now, add to the graph if and only if that edge doesnt already exist in the map
                        if write_graph[*from_id].iter().find(|(id, _)| id == to).is_some() {
                            return Err(GraphErr::EdgeAlreadyExists)
                        }
                        
                        write_graph[*from_id].push((*to, weight));
                        self.edge_counter.fetch_add(1, Ordering::SeqCst);
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
        Err(GraphErr::NoSuchNode)
    }


    // TODO: MAKE WORK
    fn update_edge(&self, from: Id, to: Id, weight: f64) -> Result<f64, GraphErr> {
        return Err(GraphErr::NoSuchNode);
    }

    fn update_or_add_edge(&self, from: Id, to: Id, weight: f64) -> Result<EdgeChange, GraphErr> {
        return Err(GraphErr::NoSuchNode);
    }

    fn add_node(&self, id: Id) -> Result<(), GraphErr> {
        let read_map = self.map.read().unwrap();
        let read_id = read_map.get(&id);

        match read_id {
            Some(_) => {
                return Err(GraphErr::NodeAlreadyExists);
            }
            None => ()
        }

        let index = self.vertex_counter.fetch_add(1, Ordering::SeqCst);

        drop(read_map);

        let mut writing = self.graph.write().unwrap();
        writing.push(vec![]);
        // let length = writing.len();
        self.map.write().unwrap().insert(id, index);
        self.id_to_value.write().unwrap().insert(index, id);
        
        
        
        Ok(())
       

    }

    // enough just to remove connections --  the id will now just be ignored, which is super space inefficient!
    // extremely slow -- sequential search 
    // messiest code of all time -- sorry!
    fn remove_node(&self, id: Id) -> Result<(), GraphErr> {
        Err(GraphErr::NoSuchEdge)
    }

    fn debug(&self) {
        ()
    }

    fn get_nodes(&self) -> Vec<Id> {
        vec!()
    }
    
    fn get_edges(&self) -> Vec<(Id, Id, f64)> {
        vec!()
    }
}


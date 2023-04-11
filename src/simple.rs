use std::sync::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use std::hash::Hash;
use crate::graph::{GraphErr,Graph};

struct SimpleGraph<Id: Hash + Eq>
{
    counter: u64,
    // map from user's id to our id
    map: RwLock<HashMap<Id, u64>>,
    graph: RwLock<Vec<Arc<RwLock<Vec<Id>>>>>,
}

impl<Id: Hash + Eq> Graph<Id> for SimpleGraph<Id> {
    fn new() -> Self {
        Self { 
            counter: 0,
            map: RwLock::new(HashMap::new()),
            graph: RwLock::new(vec![]) }
    }

    fn get_size(&self) -> (u64, u64) {
        (0, 0) // TODO is this useful
    }

    fn get_edge(&self, from: Id, to: Id) -> Result<f64, GraphErr> {
        Err(GraphErr::NoSuchEdge)
    }

    fn add_edge(&mut self, from: Id, to: Id, weight: f64) -> Result<(), GraphErr> {
        // just update both parts of the arr
        // check if an edge alr exists? kind of annoying, but necessary
        
        // first, transform to ids
        let read_map = self.map.read().unwrap();

        if let Some(v) = read_map.get(&from) {

        }
        // let _from = read_map.find(from);
        // let _to = read_map.find(to);
        

        // let read_graph = self.graph.unwrap().read();

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
        // let mut graph =

        // TODO

        Err(GraphErr::NodeAlreadyExists)
    }

    // enough just to remove connections
    // obviously, using a vector will probably be fairly slow here
    fn remove_node(&mut self, id: Id) -> Result<(), GraphErr> {
        Err(GraphErr::NoSuchNode)
    }
}

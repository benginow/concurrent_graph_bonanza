// so the key feature of graphone is that we speed up edge writes by buffering of edge inserts?
// and then we make our inserts in a batched fashion... who thought this was a good idea again?
// lmao idc
use std::fmt::Debug;
use std::hash::Hash;
use crate::graph::{EdgeChange,Graph,GraphErr};
use lazy_static::lazy_static;

lazy_static! {
    static ref LOG_SIZE: usize = env!("LOG_SIZE").parse::<usize>().unwrap();
    static ref FLUSH_LOG_AT: usize = env!("FLUSH_LOG_AT").parse::<usize>().unwrap();
}

struct GraphOne<Id: Clone + Debug + Eq + Hash> {
    log: Vec<(Id, Id)>,
    csr: ()
}

impl<Id: Clone + Debug + Eq + Hash> Graph<Id> for GraphOne<Id> {
    fn new() -> Self {
        Self {
            log: vec!(),
            csr: ()
        }
    }

    fn get_size(&self) -> (usize, usize) {
        (0, 0)
    }
    
    fn get_edge(&self, from: Id, to: Id) -> Result<f64, GraphErr> {
        Err(GraphErr::NoSuchEdge)
    }

    fn get_neighbors(&self, id: Id) -> Result<Vec<Id>, GraphErr> {
        Err(GraphErr::NoSuchNode)
    }

    fn add_edge(&mut self, from: Id, to: Id, weight: f64) -> Result<(), GraphErr> {
        Err(GraphErr::EdgeAlreadyExists)
    }

    fn remove_edge(&mut self, from: Id, to: Id) -> Result<f64, GraphErr> {
        Err(GraphErr::NoSuchEdge)
    }
    
    fn update_edge(&mut self, from: Id, to: Id, weight: f64) -> Result<f64, GraphErr> {
        Err(GraphErr::NoSuchEdge)
    }

    fn update_or_add_edge(&mut self, from: Id, to: Id, weight: f64) -> Result<EdgeChange, GraphErr> {
        Err(GraphErr::NoSuchNode)
    }
    
    fn add_node(&mut self, id: Id) -> Result<(), GraphErr> {
        Err(GraphErr::NodeAlreadyExists)
    }
    
    fn remove_node(&mut self, id: Id) -> Result<(), GraphErr> {
        Err(GraphErr::NoSuchNode)
    }

    fn debug(&self) {
        ()
    }
}

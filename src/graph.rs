use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, Eq, PartialEq)]
pub enum GraphErr {
    NoSuchNode,
    NoSuchEdge,
    NodeAlreadyExists,
    EdgeAlreadyExists,
}

pub enum EdgeChange {
    Updated(f64),
    Added,
}

pub trait Graph<Id: Clone + Debug + Eq + Hash> {
    fn new() -> Self;
    
    // returns (V, E)
    fn get_size(&self) -> (usize, usize);
    fn get_edge(&self, from: Id, to: Id) -> Result<f64, GraphErr>;
    fn get_neighbors(&self, id: Id) -> Result<Vec<Id>, GraphErr>;

    fn add_edge(&mut self, from: Id, to: Id, weight: f64) -> Result<(), GraphErr>;
    // returns old edge weight
    fn remove_edge(&mut self, from: Id, to: Id) -> Result<f64, GraphErr>;
    fn update_edge(&mut self, from: Id, to: Id, weight: f64) -> Result<f64, GraphErr>;
    // returns edge already existed ? old edge weight : zero
    fn update_or_add_edge(&mut self, from: Id, to: Id, weight: f64) -> Result<EdgeChange, GraphErr>;
    
    fn add_node(&mut self, id: Id) -> Result<(), GraphErr>;
    fn remove_node(&mut self, id: Id) -> Result<(), GraphErr>;

    fn debug(&self);
}

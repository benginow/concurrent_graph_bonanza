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
    fn new() -> Self where Self: Sized;
    
    // returns (V, E)
    fn get_size(&self) -> (usize, usize);
    fn get_edge(&self, from: Id, to: Id) -> Result<f64, GraphErr>;
    fn get_nodes(&self) -> Vec<Id>;
    fn get_edges(&self) -> Vec<(Id, Id, f64)>;
    fn get_neighbors(&self, id: Id) -> Result<Vec<Id>, GraphErr>;

    // label being f64 is kind of arbitrary here -- returns old node label
    fn get_node_label(&self, id: Id) -> Result<f64, GraphErr>;
    fn set_node_label(&self, id: Id, label: f64) -> Result<f64, GraphErr>;

    fn add_edge(&self, from: Id, to: Id, weight: f64) -> Result<(), GraphErr>;
    // returns old edge weight
    fn remove_edge(&self, from: Id, to: Id) -> Result<f64, GraphErr>;
    fn update_edge(&self, from: Id, to: Id, weight: f64) -> Result<f64, GraphErr>;
    // returns edge already existed ? old edge weight : zero
    fn update_or_add_edge(&self, from: Id, to: Id, weight: f64) -> Result<EdgeChange, GraphErr>;
    
    fn add_node(&self, id: Id) -> Result<(), GraphErr>;
    fn remove_node(&self, id: Id) -> Result<(), GraphErr>;

    fn debug(&self);
}

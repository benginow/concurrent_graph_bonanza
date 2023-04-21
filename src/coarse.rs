use std::fmt::Debug;
use std::sync::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use std::hash::Hash;
use crate::graph::{EdgeChange,GraphErr,Graph};
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct AdjList {
    adj_list: RwLock<Vec<Vec<(usize, f64)>>>,
    e: AtomicUsize
}

unsafe impl Send for AdjList {}
unsafe impl Sync for AdjList {}

impl CoarseLogList {
    fn intern_update_or_add_edge(&mut self, from: usize, to: usize, weight: f64, can_update: bool, can_add: bool) -> Result<EdgeChange, GraphErr> {
        let mut adj = self.adj_list.write().unwrap();

        if can_update {
            for (t_, w) in adj_list[from].iter() {
                if *t_ == to {
                    let w_ = *w;
                    *w = weight; // ?
                    return Ok(EdgeChange::Updated(w_));
                }
            }
        }

        if can_add {
            adj_list[from].push((to, weight));
        }

        Err(GraphErr::NoSuchEdge)
    }
}

impl Graph<usize> for AdjList {
    fn new() -> Self {
        Self { 
            adj: RwLock::new(vec!()),
            e: AtomicUsize::new(0)
        }
    }

    // returns (V, E)
    fn get_size(&self) -> (usize, usize);
    fn get_edge(&self, from: usize, to: usize) -> Result<f64, GraphErr>;
    fn get_nodes(&self) -> Vec<usize>;
    fn get_edges(&self) -> Vec<(usize, usize, f64)>;
    fn get_neighbors(&self, id: usize) -> Result<Vec<usize>, GraphErr>;

    // label being f64 is kind of arbitrary here -- returns old node label
    fn get_node_label(&self, id: usize) -> Result<f64, GraphErr>;
    fn set_node_label(&self, id: usize, label: f64) -> Result<f64, GraphErr>;

    fn add_edge(&self, from: usize, to: usize, weight: f64) -> Result<(), GraphErr>;
    // returns old edge weight
    fn remove_edge(&self, from: usize, to: usize) -> Result<f64, GraphErr>;
    fn update_edge(&self, from: usize, to: usize, weight: f64) -> Result<f64, GraphErr>;
    // returns edge already existed ? old edge weight : zero
    fn update_or_add_edge(&self, from: usize, to: usize, weight: f64) -> Result<EdgeChange, GraphErr>;
    
    fn add_node(&self, id: usize) -> Result<(), GraphErr>;
    fn remove_node(&self, id: usize) -> Result<(), GraphErr>;

    fn debug(&self);
}

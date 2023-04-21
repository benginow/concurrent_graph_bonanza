use std::fmt::Debug;
use std::sync::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use std::hash::Hash;
use crate::graph::{EdgeChange,GraphErr,Graph};
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct AdjList {
    adj: RwLock<Vec<Vec<(usize, f64)>>>,
    e: AtomicUsize
}

unsafe impl Send for AdjList {}
unsafe impl Sync for AdjList {}

impl Graph<usize> for AdjList {
    fn new() -> Self {
        Self { 
            adj: RwLock::new(vec!()),
            e: AtomicUsize::new(0)
        }
    }
}

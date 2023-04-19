use std::fmt::Debug;
use std::hash::Hash;
use crate::graph::{EdgeChange,Graph,GraphErr};
use std::sync::{Arc,RwLock,RwLockWriteGuard,atomic::{AtomicUsize, Ordering}};
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct List {
    adj_list: Vec<Vec<(usize, f64)>>,
    e: AtomicUsize
}

impl List {
    fn intern_update_or_add_edge(&mut self, from: usize, to: usize, weight: f64, can_add: bool, can_update: bool) -> Result<EdgeChange, GraphErr> {
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


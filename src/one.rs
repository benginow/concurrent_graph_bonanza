// so the key feature of graphone is that we speed up edge writes by buffering of edge inserts?
// and then we make our inserts in a batched fashion... who thought this was a good idea again?
// lmao idc
use std::fmt::Debug;
use std::hash::Hash;
use crate::graph::{EdgeChange,Graph,GraphErr};
use std::sync::{Arc,RwLock,RwLockWriteGuard,atomic::{AtomicUsize, Ordering}};
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    // static ref LOG_SIZE: usize = env!("LOG_SIZE").parse::<usize>().unwrap();
    // static ref FLUSH_LOG_AT: usize = env!("FLUSH_LOG_AT").parse::<usize>().unwrap();
}

#[derive(Debug)]
pub struct CoarseLogList {
    log: RwLock<Vec<(usize, usize, f64, bool)>>,
    adj_list: RwLock<Vec<Vec<(usize, f64)>>>,
    e: AtomicUsize
}

impl CoarseLogList {
    fn intern_update_edge() {

    }

    fn intern_add_edge() {
        
    }

    fn flush (&mut self, log: RwLockWriteGuard<Vec<(usize, usize, f64, bool)>>) {
        
    }
    
    fn intern_update_or_add_edge(&mut self, from: usize, to: usize, weight: f64, is_update: bool) {
        let mut log = (&mut *self).log.write().unwrap();
        /*
        if log.len() == *FLUSH_LOG_AT {
            self.flush(log);
        } else {
            log.push(
                (
                    from,
                    to,
                    weight,
                    is_update
                )
            );
        }
        */
    }
}

impl Graph<usize> for CoarseLogList {
    fn new() -> Self {
        Self {
            log: RwLock::new(vec!()),
            adj_list: RwLock::new(vec!()),
            e: AtomicUsize::new(0)
        }
    }

    fn get_size(&self) -> (usize, usize) {
        (self.adj_list.read().unwrap().len(), self.e.load(Ordering::SeqCst))
    }

    fn get_edge(&self, from: usize, to: usize) -> Result<f64, GraphErr> {
        // TODO
        let list = self.adj_list.read().unwrap();
        let v = list.len();
        if v > from && v > to {
            let edges = &list[from];
            for (to_, w) in edges.iter() {
                if *to_ == to {
                    return Ok(w.clone());
                }
            }
            
            Err(GraphErr::NoSuchEdge)
        } else {
            Err(GraphErr::NoSuchNode)
        }
    }

    fn get_neighbors(&self, id: usize) -> Result<Vec<usize>, GraphErr> {
        // TODO
        let list = self.adj_list.read().unwrap();
        if list.len() > id {
            Ok(
                list[id].clone()
                    .into_iter()
                    .map(
                        |(to, _)| {
                            to
                        }
                    )
                    .collect()
            )
        } else {
            Err(GraphErr::NoSuchNode)
        }
    }

    fn get_node_label(&self, id: usize) -> Result<f64, GraphErr> {
        // TODO
        Ok(0.0)
    }

    fn set_node_label(&self, id: usize, label: f64) -> Result<f64, GraphErr> {
        // TODO
        Ok(0.0)
    }

    fn add_edge(&mut self, from: usize, to: usize, weight: f64) -> Result<(), GraphErr> {
        // TODO improve performance lol
        match self.get_edge(from, to) {
            Ok(_) => Err(GraphErr::EdgeAlreadyExists),
            Err(e) => match e {
                GraphErr::NoSuchEdge => {
                    self.intern_update_or_add_edge(from, to, weight, false);
                    Ok(())
                },
                _ => Err(e)
            }
        }
    }
    
    fn remove_edge(&mut self, from: usize, to: usize) -> Result<f64, GraphErr> {
        // TODO
        Err(GraphErr::NoSuchEdge)
    }
    
    fn update_edge(&mut self, from: usize, to: usize, weight: f64) -> Result<f64, GraphErr> {
        // TODO
        match self.get_edge(from, to) {
            Ok(w) => {
                self.intern_update_or_add_edge(from, to, weight, true);
                Ok(w)
            }
            Err(e) => Err(e)
        }
    }
    
    fn update_or_add_edge(&mut self, from: usize, to: usize, weight: f64) -> Result<EdgeChange, GraphErr> {
        // TODO
        match self.get_edge(from, to) {
            Ok(w) => {
                self.intern_update_or_add_edge(from, to, weight, true);
                Ok(EdgeChange::Updated(w))
            },
            Err(e) => match e {
                GraphErr::NoSuchEdge => {
                    self.intern_update_or_add_edge(from, to, weight, false);
                    Ok(EdgeChange::Added)
                },
                _ => Err(e)
            }
        }
    }
    
    fn add_node(&mut self, id: usize) -> Result<(), GraphErr> {
        // TODO
        let mut list = self.adj_list.write().unwrap();
        if id != list.len() {
            panic!("bad arg to add_node in coarseloglist");
        }

        list.push(vec!());
        Ok(())
    }

    fn remove_node(&mut self, id: usize) -> Result<(), GraphErr> {
        // TODO
        Err(GraphErr::NoSuchNode)
    }

    fn debug(&self) {
        // TODO i guess
        ()
    }
}

pub struct CoarseGraphOne<Id: Clone + Debug + Eq + Hash> {
    log_list: Arc<CoarseLogList>,
    internal_ids: Arc<RwLock<HashMap<Id, usize>>>
}

impl<Id: Clone + Debug + Eq + Hash> Graph<Id> for CoarseGraphOne<Id> {
    fn new() -> Self {
        Self {
            log_list: Arc::new(CoarseLogList::new()),
            internal_ids: Arc::new(RwLock::new(HashMap::new()))
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

    fn get_node_label(&self, id: Id) -> Result<f64, GraphErr> {
        Err(GraphErr::NoSuchNode)
    }

    fn set_node_label(&self, id: Id, label: f64) -> Result<f64, GraphErr> {
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

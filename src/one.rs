// so the key feature of graphone is that we speed up edge writes by buffering of edge inserts?
// and then we make our inserts in a batched fashion... who thought this was a good idea again?
// lmao idc
use std::fmt::Debug;
use std::hash::Hash;
use crate::graph::{EdgeChange,Graph,GraphErr};
use std::sync::{Arc,RwLock,RwLockWriteGuard,atomic::{AtomicUsize, Ordering}};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::atomic::AtomicPtr;
use std::marker::{Send,Sync};
use std::cell::Cell;

#[derive(Debug)]
pub struct CoarseLogList {
    log: RwLock<Vec<(usize, usize, f64, bool)>>,
    adj_list: RwLock<Vec<Vec<(usize, f64)>>>,
    e: AtomicUsize,
    log_size: AtomicUsize,
}

/*
unsafe impl Send for CoarseLogList {}
unsafe impl Sync for CoarseLogList {}
*/

impl CoarseLogList {
    fn intern_update_or_add_edge(&mut self, from: usize, to: usize, weight: f64, is_update: bool) {
        let mut log = self.log.write().unwrap();
        if log.len() == self.log_size.load(Ordering::SeqCst) {
            let mut adj = self.adj_list.write().unwrap();
            log.iter().map(|(f,t,w,u)| {
                if *u {
                    for (t_, w_) in adj[from].iter_mut() {
                        if *t_ == *t {
                            *w_ = *w;
                            break;
                        }
                    }
                } else {
                    adj[from].push((*t, *w));
                }
            });
        } else {
            log.push(
                (
                    from,
                    to,
                    weight,
                    is_update
                )
            );
            if is_update {
                self.e.fetch_add(1, Ordering::SeqCst);
            }
        }
    }
}

impl CoarseLogList {
    fn new(ls: usize) -> Self {
        Self {
            log: RwLock::new(vec!()), // TODO make this size ls
            adj_list: RwLock::new(vec!()),
            e: AtomicUsize::new(0),
            log_size: AtomicUsize::new(ls),
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

    fn get_nodes(&self) -> Vec<usize> {
        (0..self.adj_list.read().unwrap().len()).collect()
    }
    
    fn get_edges(&self) -> Vec<(usize, usize, f64)> {
        vec!()
    }

    fn get_neighbors(&self, id: usize) -> Result<Vec<usize>, GraphErr> {
        // TODO
        let list = self.adj_list.read().unwrap();
        if list.len() > id {
            Ok(
                list[id]
                    .iter()
                    .map(
                        |(to, _)| {
                            *to
                        }
                    )
                    .collect()
            )
        } else {
            Err(GraphErr::NoSuchNode)
        }
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

    fn debug(&self) {
        // TODO i guess
        ()
    }
}

pub struct CoarseGraphOne<Id: Clone + Debug + Eq + Hash> {
    log_list: Cell<CoarseLogList>,
    internal_ids: Arc<RwLock<HashMap<Id, usize>>>
}

impl<Id: Clone + Debug + Eq + Hash> CoarseGraphOne<Id> {
    fn get_id(&self, id_: &Id) -> Result<usize, GraphErr> {
        let map = self.internal_ids.read().unwrap();
        
        match map.get(id_) {
            Some(id) => Ok(id.clone()),
            _ => Err(GraphErr::NoSuchNode),
        }
    }

    fn get_ids(&self, id0_: &Id, id1_: &Id) -> Result<(usize, usize), GraphErr> {
        let map = self.internal_ids.read().unwrap();
        
        match (map.get(id0_), map.get(id1_)) {
            (Some(id0), Some(id1)) => Ok((id0.clone(), id1.clone())),
            _ => {
                Err(GraphErr::NoSuchNode)
            }
        }
    }
}

impl<Id: Clone + Debug + Eq + Hash> Graph<Id> for CoarseGraphOne<Id> {
    fn new() -> Self {
        Self {
            // log_list: Arc::new(CoarseLogList::new(100)),
            log_list: Cell::new(CoarseLogList::new(100)),
            internal_ids: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    fn get_size(&self) -> (usize, usize) {
        self.log_list.get_mut().get_size()
    }
    
    fn get_edge(&self, from: Id, to: Id) -> Result<f64, GraphErr> {
        match self.get_ids(&from, &to) {
            Ok((f_, t_)) => {
                self.log_list.get_mut().get_edge(f_, t_)
            },
            Err(e) => Err(e)
        }
    }

    fn get_nodes(&self) -> Vec<Id> {
        vec!()
    }
    
    fn get_edges(&self) -> Vec<(Id, Id, f64)> {
        vec!()
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
    
    fn add_edge(&self, from: Id, to: Id, weight: f64) -> Result<(), GraphErr> {
        match self.get_ids(&from, &to) {
            Ok((f_, t_)) => {
                self.log_list.get_mut().add_edge(f_, t_, weight)
            },
            Err(e) => Err(e)
        }
    }

    fn remove_edge(&self, from: Id, to: Id) -> Result<f64, GraphErr> {
        Err(GraphErr::NoSuchEdge)
    }
    
    fn update_edge(&self, from: Id, to: Id, weight: f64) -> Result<f64, GraphErr> {
        match self.get_ids(&from, &to) {
            Ok((f_, t_)) => {
                self.log_list.get_mut().update_edge(f_, t_, weight)
            },
            Err(e) => Err(e)
        }
    }

    fn update_or_add_edge(&self, from: Id, to: Id, weight: f64) -> Result<EdgeChange, GraphErr> {
        match self.get_ids(&from, &to) {
            Ok((f_, t_)) => {
                self.log_list.get_mut().update_or_add_edge(f_, t_, weight)
            },
            Err(e) => Err(e)
        }
    }
    
    fn add_node(&self, id: Id) -> Result<(), GraphErr> {
        match self.get_id(&id) {
            Ok(_) => {
                Err(GraphErr::NodeAlreadyExists)
            },
            _ => {
                let mut ids = self.internal_ids.write().unwrap();

                let (v, _) = self.get_size();
                
                ids.insert(id, v);
                self.log_list.get_mut().add_node(v)
            }
        }
    }

    fn remove_node(&self, id: Id) -> Result<(), GraphErr> {
        Err(GraphErr::NoSuchNode)
    }
    
    fn debug(&self) {
        ()
    }
}

unsafe impl<Id: Clone + Debug + Eq + Hash> Send for CoarseGraphOne<Id> {}
unsafe impl<Id: Clone + Debug + Eq + Hash> Sync for CoarseGraphOne<Id> {}

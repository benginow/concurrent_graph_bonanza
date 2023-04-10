use std::sync::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use std::hash::{Hash};


fn main() {
    println!("Hello, world!");
}



struct SimpleGraph<N: Hash + Eq> 
    {
    // map from object to id
    counter: u64,
    map: RwLock<HashMap<N, u64>>,
    graph: RwLock<Vec<Arc<RwLock<Vec<N>>>>>,
}

impl<N: Hash + Eq> SimpleGraph<N> {
    fn new() -> Self {
        Self { 
            counter: 0,
            map: RwLock::new(HashMap::new()),
            graph: RwLock::new(vec![]) }
    }

    fn add_edge(&mut self, id: N, id2: N) -> bool {
        // just update both parts of the arr
        // check if an edge alr exists? kind of annoying, but necessary
        
        // first, transform to ids
        let read_map = self.map.read().unwrap();

        if let Some(v) = read_map.get(&id) {

        }
        // let _id = read_map.find(id);
        // let _id2 = read_map.find(id2);
        

        // let read_graph = self.graph.unwrap().read();

        return false;
        
        
    }
    
    fn remove_edge() {

    }

    fn add_node() -> u64 {
        // let mut graph = 
    }

    // enough just to remove connections
    // obviously, using a vector will probably be fairly slow here
    fn remove_node() {

    }
}
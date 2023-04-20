// use std::fs;
// use crate::simple::SimpleGraph;
// use crate::graph::Graph;
// use crate::coarse::CoarseCSRGraph;
// use crate::graph::GraphErr;

// use std::hash::Hash;
// use std::collections::HashMap;
// use std::sync::Arc;
// use std::sync::mpsc::{Sender, Receiver};
// use std::sync::mpsc;
// use std::thread;

// static NUM_THREADS: u32 = 8;

// fn relax_edge<Id: Clone + Eq + Hash + std::fmt::Debug, G: Graph<Id>>(graph: G, from: Id, to: Id){
//     // read
//     // let val1 = g.
//     // let val2 =  
//     // if 
// }

// fn scheduler<Id: Clone + Eq + Hash + std::fmt::Debug>(r: Receiver<(Id,Id)>, snds: Vec<Sender<(Id, Id)>>) {
//     // basically, just schedule round-robin
//     // could be slow.. hmm
//     let curr_thread = 0;

// }

// fn per_thread<Id: Clone + Eq + Hash + std::fmt::Debug, G: Graph<Id>>(graph: G, source: Id) {
//     // each thread will wait on something from the channel, and then relax the edge
    
// }

// // gather distances that have been set
// fn gather_distances<Id: Clone + Eq + Hash + std::fmt::Debug, G: Graph<Id>>(graph: Arc<G>, source: Id) {

// }

// // hashmap of IDs to their corresponding lengths
// fn sssp<Id: Clone + Eq + Hash + std::fmt::Debug, G: Graph<Id>>(graph: Arc<G>, source: Id) -> HashMap<Id, f64> {
//     let visited = HashSet::new();
//     let final_mapping = HashMap::new();


//     let (scheduler_sender, scheduler_receiver): (Sender<(Id, Id)>, Receiver<(Id, Id)>) = mpsc::channel();


//     return final_mapping;

    


//     // all node length init to infinity -- just check if there is anything in final mapping


// }

// // non sequential, compare to output from sssp
// fn dijkstra<Id: Clone + Eq + Hash + std::fmt::Debug, G: Graph<Id>>(graph: G){
    
// }
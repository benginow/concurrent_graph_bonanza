use std::fs;
use crate::simple::SimpleGraph;
use crate::graph::Graph;
use crate::coarse::CoarseCSR;
use crate::graph::GraphErr;
use std::hash::Hash;
use std::collections::HashMap;



// hashmap of IDs to their corresponding lengths
fn sssp<Id: Clone + Eq + Hash, G: Graph<Id>>(graph: G, source: Id) -> HashMap<Id, f64> {
    let final_mapping = HashMap::new();
    return final_mapping;
}
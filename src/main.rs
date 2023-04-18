pub mod graph;
pub mod simple;
pub mod coarse;

use std::fs;

use crate::simple::SimpleGraph;
use crate::graph::Graph;
use crate::coarse::CoarseCSR;
use crate::graph::GraphErr;



fn main() {
    
}

#[cfg(test)]
fn test_simple () {
    let mut g: SimpleGraph<usize> = SimpleGraph::new();
    make_sure_graph_works(g);
    // g = SimpleGraph::new();
    // make_sure_graph_works_concurrent(g);
}

#[cfg(test)]
fn test_coarse() {
    let mut g: CoarseCSR = CoarseCSR::new();
    make_sure_graph_works(g);
    // g = CoarseCSR::new();
    // make_sure_graph_works_concurrent(g);
}

// not concurrenct
fn make_sure_graph_works<G: Graph<usize>>(mut g: G) {
    // make sure that adding any arbitrary number of entries works
    let mut val: Result<(), GraphErr>;
    for i in 0..5 {
        val = g.add_node(i);
        assert!(val.is_ok());
    }

    // don't allow for duplicate entries.
    val = g.add_node(3);
    assert!(val.is_err());

    // make sure size is correct
    assert!(g.get_size() ==(4, 0));

    // make sure double removal is not a thing
    val = g.remove_node(3);
    assert!(val.is_ok());
    val = g.remove_node(3);
    assert!(val.is_err());

    for i in 0..5 {
        for j in 0..5 {
            val = g.add_edge(i,j,1.0);
            assert!(val.is_ok());
        }
    }

    let mut grapherr = g.remove_edge(3, 3);
    assert!(grapherr.is_ok());
    grapherr = g.remove_edge(3, 3);
    assert!(grapherr.is_err());

    upodate_or_add_edge(2,3,.5);





}


fn make_sure_graph_works_concurrent<G: Graph<usize>>(mut g: G) {
    // we want to make sure that accesses to the graph can agree on some consistent
    // assertions about the state

    // how can we make sure that these assertions are correct?

    // might just impl boruvka's for this

}


// _____ TESTS IN PROGRESS ______

// sequentially add to graph
// fn build_test_graph_parallel() -> SimpleGraph<int> {
//     let file_contents = fs::read_to_string("test1.txt").expect("unable to read file");
//     let lines = file_contents.lines();

//     // spawn n threads -- maybe get input from cmd line at this point
//     let handles = vec![];
//     thread::spawn(|| {

//     })
//     // for line in file_contents.lines(){

//     // }
// }

fn boruvka() {
    // keep an event queue for nodes

}

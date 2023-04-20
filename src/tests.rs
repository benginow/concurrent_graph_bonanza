#[cfg(test)]

use std::fs;
use crate::simple::SimpleGraph;
use crate::graph::Graph;
// use crate::coarse::CoarseCSRGraph;
use crate::graph::GraphErr;


#[test]
fn test_simple() {
    let mut g: SimpleGraph<usize> = SimpleGraph::new();
    make_sure_graph_works(g);
}

// #[test]
// fn test_coarse() {
//     let mut g: CoarseCSRGraph<usize> = CoarseCSRGraph::new();
//     make_sure_graph_works(g);
// }

// not concurrenct
fn make_sure_graph_works<G: Graph<usize>>(mut g: G) {
    // make sure that adding any arbitrary number of entries works
    let mut val: Result<(), GraphErr>;
    for i in 0..5 {
        val = g.add_node(i);
        assert!(val.is_ok());
        print!("hi");
    }
    
    // don't allow for duplicate entries.
    val = g.add_node(3);
    assert!(val.is_err());
    
    print!("hi");

    // make sure size is correct
    println!("{0:?}", g.get_size());
    assert!(g.get_size() == (5, 0));
    print!("hi");
    
    // make sure double removal is not a thing
    val = g.remove_node(3);
    assert!(val.is_ok());
    assert!(g.get_size() == (4, 0));
    g.debug();
    val = g.remove_node(3);
    assert!(val.is_err());
    assert!(g.get_size() == (4, 0));
    g.debug();

    let mut nedges = 0;
    for i in 0..5 {
        for j in 0..5 {
            println!("add edge {} {} {}", i, j, 1.0);
            val = g.add_edge(i,j,1.0);
            g.debug();
            println!("result is {:?}", val);
            if i != 3 && j != 3 {
                assert!(val.is_ok());
                nedges += 1;
            } else {
                assert!(val == Err(GraphErr::NoSuchNode))
            }
        }
    }

    assert!(g.get_size() == (4, nedges));
    let mut val_ = g.remove_edge(2, 2);
    assert!(val_.is_ok());
    assert!(g.get_size() == (4, nedges - 1));
    val_ = g.remove_edge(2, 2);
    assert!(val_ == Err(GraphErr::NoSuchEdge));
    assert!(g.get_size() == (4, nedges - 1));

    assert!(g.get_edge(2, 4) == Ok(1.0));
    g.update_or_add_edge(2, 4, 0.5);
    assert!(g.get_edge(2, 4) == Ok(0.5));

    let value = g.remove_node(1);
    assert!(value.is_ok());


    
}


/*
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
*/

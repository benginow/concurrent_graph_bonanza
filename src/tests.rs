#[cfg(test)]

use std::fs;
// use crate::simple::SimpleGraph;
use crate::graph::Graph;
use crate::csr::CoarseCSRGraph;
use crate::graph::GraphErr;
use std::boxed::Box;
use std::thread;
use std::thread::JoinHandle;
use std::cell::Cell;
use std::sync::Arc;

/*
#[test]
fn test_simple() {
    let mut g: SimpleGraph<usize> = SimpleGraph::new();
    make_sure_graph_works(g);
}
*/

#[test]
fn test_coarse() {
    let mut g: CoarseCSRGraph<usize> = CoarseCSRGraph::new();
    make_sure_graph_works(g);
    // TODO custom test
}

#[test]
fn test_coarse_concurrent() {
    let mut g: CoarseCSRGraph<usize> = CoarseCSRGraph::new();
    let a = Arc::new(g);
    make_sure_graph_works_concurrent(a);
}

pub fn tc() {
    let mut g: CoarseCSRGraph<usize> = CoarseCSRGraph::new();
    let a = Arc::new(g);
    make_sure_graph_works_concurrent(a);
}

pub fn tcc() {
    let mut g: CoarseCSRGraph<usize> = CoarseCSRGraph::new();
    let a = Arc::new(g);
    make_sure_graph_works_concurrent(a);
}

// not concurrent
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
    assert!(g.get_size() == (5, 0));
    
    // make sure double removal is not a thing
    val = g.remove_node(3);
    assert!(val.is_ok());
    assert!(g.get_size() == (4, 0));
    val = g.remove_node(3);
    assert!(val.is_err());
    assert!(g.get_size() == (4, 0));

    let mut nedges = 0;
    for i in 0..5 {
        for j in 0..5 {
            val = g.add_edge(i,j,1.0);
            if i != 3 && j != 3 {
                assert!(val.is_ok());
                nedges += 1;
            } else {
                assert!(val == Err(GraphErr::NoSuchNode))
            }
        }
    }

    g.debug();
    
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
struct Wrapper<G: Graph<usize> + Send + Sync> {
    pub g: Cell<G>
}

impl<G: Graph<usize> + Send + Sync> Wrapper<G> {
    fn new(g: G) -> Self {
        Self {
            g: Cell::new(g)
        }
    }
}

unsafe impl<G: Graph<usize> + Send + Sync> Send for Wrapper<G> {}
unsafe impl<G: Graph<usize> + Send + Sync> Sync for Wrapper<G> {}


fn make_sure_graph_works_concurrent<G: Graph<usize> + Send + Sync> (mut g: G) {
    let w = Wrapper::new(g);
    
    let mut handles = vec!();
    for i in 0..10 {
        // let w_: *const Wrapper<G> = &w;
        handles.push(thread::spawn(|| {
            w.g.get_mut().add_node(i);
        }));
    }

    for h in handles.iter() {
        h.join();
    }
}
*/

fn make_sure_graph_works_concurrent<G: Graph<usize> + Send + Sync + 'static> (a: Arc<G>) {
    let mut handles: Vec<JoinHandle<()>> = vec!();
    
    println!("hi");
    for i in 0..10 {
        let a_ = a.clone();
        let i_ = i.clone();
        handles.push(thread::spawn(move || {
            println!("hey from thread {}", i);
            a_.add_node(i_);
            a_.debug();
        }));
    }

    let mut i = 0;
    for h in handles {
        println!("JOIN {}", i);
        i += 1;
        h.join();
    }
    println!("MAIN EXIT");
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

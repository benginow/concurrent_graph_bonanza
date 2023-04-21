#[cfg(test)]

use std::fs;
use std::sync::mpsc::{channel, Sender, Receiver};
use rand::Rng;
use std::sync::atomic::{AtomicUsize, Ordering};


use crate::simple::SimpleGraph;
use crate::one::CoarseGraphOne;
use crate::graph::Graph;
use crate::coarse::CoarseCSRGraph;
use crate::graph::GraphErr;
use std::boxed::Box;
use std::thread;
use std::thread::JoinHandle;
use std::cell::Cell;
use std::sync::Arc;
use std::time::Instant;


// #[test]
// fn test_simple_seq() {
//     let mut g: SimpleGraph<usize> = SimpleGraph::new();
//     make_sure_graph_works(g);
// }

// #[test]
// fn test_coarse_seq() {
//     let mut g: CoarseCSRGraph<usize> = CoarseCSRGraph::new();
//     make_sure_graph_works(g);
// }


// ________________________________________PARALLEL TESTS BEGIN HERE________________________________________

enum RequestType {
    AddNode(usize),
    RemoveNode(usize),
    AddEdge(usize, usize),
    RemoveEdge(usize, usize),
    Done
}

// don't make generic for now bc life too hard already
fn service<G: Graph<usize> + Send + Sync>(rx: Receiver<RequestType>, graph: Arc<G>) {
    loop {
        let request = rx.recv();
        // fail silently..
        if (request.is_err()) {
            print!("FAILING THREAD SILENTLY. PROCEED WITH CAUTION");
            break;
        }
        let request = request.unwrap();
        match request {
            RequestType::Done => { break; }
            RequestType::AddNode(a) => {
                graph.add_node(a);
            }
            RequestType::RemoveNode(a) => {
                graph.remove_node(a);
            }
            RequestType::AddEdge(a,b) => {
                // weight doesn't super matter here, just pass in 1.0
                graph.add_edge(a,b, 1.0);
            }
            RequestType::RemoveEdge(a, b) => {
                graph.remove_edge(a, b);
            }

        }
    }

}

fn generate_request(num: usize, nodes_gen: &AtomicUsize) -> RequestType {
    let mut num = num.clone();
    loop {
        match num {
            0 => {
                let val = nodes_gen.fetch_add(1, Ordering::SeqCst);
                // print!("{val}");
                return RequestType::AddNode(val);
            }
            1 => {
                let val = nodes_gen.load(Ordering::SeqCst);
                if (val == 0) {
                    num = 0;
                    continue;
                }
                let mut rng = rand::thread_rng();
                // print!("{val}");
                let node1 = rng.gen_range(0..val);
                let node2 = rng.gen_range(0..val);
                return RequestType::AddEdge(node1, node2);
            }
            2 => {
                let val = nodes_gen.load(Ordering::SeqCst);
                if (val == 0) {
                    num = 0;
                    continue;
                }
                let mut rng = rand::thread_rng();
                // print!("{val}");
                let node = rng.gen_range(0..val);
                return RequestType::RemoveNode(node);

            }
            _ => {
                let val = nodes_gen.load(Ordering::SeqCst);
                if (val == 0) {
                    num = 0;
                    continue;
                }
                // print!("{val}");
                let mut rng = rand::thread_rng();
                let node1 = rng.gen_range(0..val);
                let node2 = rng.gen_range(0..val);
                return RequestType::RemoveEdge(node1, node2);
            }

        }
    }  
}

// graph must impl send and sync :)
fn test_gen<G: Graph<usize> + Send + Sync + 'static>(graph: Arc<G>, nodes: &mut usize, edges: &mut usize, removed_nodes: &mut usize, removed_edges: &mut usize, num_threads: usize)
{
    // keep track of node ids, very very very roughly, ignoring removals to avoid memory overhead
    let nodes_generated = AtomicUsize::new(0);

    //start the workers, set up channel
    let mut sender_list: Vec<Sender<RequestType>> = vec![];
    let mut handles: Vec<JoinHandle<_>> = vec![];
    for i in 0..num_threads {
        let (tx, rx) = channel();
        sender_list.push(tx);
        let g = Arc::clone(&graph);
        handles.push(
            thread::spawn(move || {
                service(rx, g);
        }));
    }

    // give threads work in a round-robin fashion
    let curr_thread = 0;

    while (*nodes + *edges + *removed_edges + *removed_nodes > 0){
        let mut rng = rand::thread_rng();
        let req_type = rng.gen_range(0..4);
        let req = generate_request(req_type, &nodes_generated);
        match req {
            RequestType::AddNode(_) => {
                if (*nodes > 0){
                    let res = sender_list[curr_thread].send(req);
                    if (res.is_err()){
                        print!("oh shit!");
                        return;
                    }
                    *nodes = *nodes - 1;
                }
            }
            RequestType::RemoveNode(_) => {
                if (*removed_nodes > 0){
                    let res = sender_list[curr_thread].send(req);
                    if (res.is_err()){
                        print!("oh shit!");
                        return;
                    }
                    *removed_nodes = *removed_nodes - 1;
                }

            }
            RequestType::AddEdge(_,_) => {
                if (*edges > 0){
                    let res = sender_list[curr_thread].send(req);
                    if (res.is_err()){
                        print!("oh shit!");
                        return;
                    }
                    *edges = *edges - 1;
                }
                
            }
            RequestType::RemoveEdge(_,_) => {
                if (*removed_edges > 0){
                    let res = sender_list[curr_thread].send(req);
                    if (res.is_err()){
                        print!("oh shit!");
                        return;
                    }
                    *removed_edges = *removed_edges - 1;
                    
                }
            }
            _ => ()
        }
    }

    // cleanup
    for i in 0..sender_list.len() {
        sender_list[i].send(RequestType::Done);
    }

    for handle in handles {
        handle.join();
    }

}

fn bench<G: Graph<usize> + Send + Sync + 'static>(g: Arc<G>, num_nodes: &mut usize, num_edges: &mut usize, removed_nodes: &mut usize, removed_edges: &mut usize, num_threads: usize){
    let mut timing = 0.0;
    let num_loops = 15;
    let div_me = (*num_edges + *num_nodes) as u128;
    // 1024 by 1024
    for i in 0..num_loops{
        let start = Instant::now();
        
        test_gen(g.clone(), num_nodes, num_edges, removed_nodes, removed_edges, num_threads);
        let duration = start.elapsed().as_micros();
        timing += duration as f64 / num_loops as f64;
    }
    let throughput = div_me as f64 / timing as f64;
    println!("e: {num_edges} n: {num_nodes} t: {num_threads} duration: {timing} throughput(instr/micros) = {throughput}");
}

#[test]
fn test_simple_conc_vary_num_threads() {
    println!("VARYING NUM THREADS");
    for i in 0..8 {
        let g: Arc<SimpleGraph<usize>> = Arc::new(SimpleGraph::new());
        bench(g, &mut 1024, &mut 1024, &mut 0, &mut 0, 1 << i);
    }
}

#[test]
fn test_coarse_csr_conc_vary_num_threads() {
    println!("VARYING NUM THREADS");
    for i in 0..8 {
        let g: Arc<CoarseCSRGraph<usize>> = Arc::new(CoarseCSRGraph::new());
        bench(g, &mut 1024, &mut 1024, &mut 0, &mut 0, 1 << i);
    }
}


#[test]
fn test_graph_one_vary_num_threads() {
    println!("VARYING NUM THREADS");
    for i in 0..8 {
        let g: Arc<CoarseGraphOne<usize>> = Arc::new(CoarseGraphOne::new());
        bench(g, &mut 1024, &mut 1024, &mut 0, &mut 0, 1 << i);
    }
}

#[test]
fn test_coarse_conc() {
    let mut g: Arc<CoarseCSRGraph<usize>> = Arc::new(CoarseCSRGraph::new());
    test_gen(g, &mut 1024, &mut 1024, &mut 0, &mut 0, 5);
}

// ____________________________________ SEQUENTIAL TESTS -- UNCOMMENT IF NEEDED ____________________________________

// fn make_sure_graph_works<G: Graph<usize>>(mut g: G) {
//     // make sure that adding any arbitrary number of entries works
//     let mut val: Result<(), GraphErr>;
//     for i in 0..5 {
//         val = g.add_node(i);
//         assert!(val.is_ok());
//     }
    
//     // don't allow for duplicate entries.
//     val = g.add_node(3);
//     print!("value:{val:?}\n");
//     assert!(val.is_err());
    
//     // make sure size is correct
//     assert!(g.get_size() == (5, 0));
    
    // make sure double removal is not a thing

    // val = g.remove_node(3);
    // assert!(val.is_ok());
    // assert!(g.get_size() == (4, 0));
    // g.debug();
    // val = g.remove_node(3);
    // assert!(val.is_err());
    // assert!(g.get_size() == (4, 0));
    // g.debug();

    // let mut nedges = 0;
    // for i in 0..5 {
    //     for j in 0..5 {
    //         println!("add edge {} {} {}", i, j, 1.0);
    //         val = g.add_edge(i,j,1.0);
    //         g.debug();
    //         println!("result is {:?}", val);
    //         if i != 3 && j != 3 {
    //             assert!(val.is_ok());
    //             nedges += 1;
    //         } else {
    //             assert!(val == Err(GraphErr::NoSuchNode))
    //         }
    //     }
    // }

    // assert!(g.get_size() == (4, nedges));
    // let mut val_ = g.remove_edge(2, 2);
    // assert!(val_.is_ok());
    // assert!(g.get_size() == (4, nedges - 1));
    // val_ = g.remove_edge(2, 2);
    // assert!(val_ == Err(GraphErr::NoSuchEdge));
    // assert!(g.get_size() == (4, nedges - 1));

    // assert!(g.get_edge(2, 4) == Ok(1.0));
    // g.update_or_add_edge(2, 4, 0.5);
    // assert!(g.get_edge(2, 4) == Ok(0.5));

    // let value = g.remove_node(1);
    // assert!(value.is_ok());


    
// }


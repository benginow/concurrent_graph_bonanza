use crate::graph::{Graph,GraphErr};
use disjoint_sets::AUnionFind;
use std::collections::HashMap;

// i'm lazy so these all assume that the input graphs are connected
fn kruskal<G: Graph<usize>> (mut g: G) -> G {
    let nodes = g.get_nodes();
    let edges = g.get_edges();
    let mut mst = G::new();
    for v in nodes {
        mst.add_node(v);
    }
    
    edges.sort_by_key(
        |(_, _, w)| w
    );

    let uf = UnionFind::new(nodes.len());
    for v in nodes.iter() {
        uf.alloc(v);
    }
    
    for (from, to, w) in edges.iter() {
        if !uf.equiv(from, to) {
            uf.union(from, to);
            mst.add_edge(from, to, w);
        }
    }

    mst
}

/*
fn dumb_boruvka<G: Graph<usize>> (mut g: G) -> G {
    let nodes = g.get_nodes();
    let mut mst = G::new();

    // edges sorted in reverse order by weight
    let mut edges: Vec<Vec<(usize, f64)>> = nodes.iter().map(|| {
        vec!()
    }).collect();
    g.get_edges().iter().map(|(f, t, w)| {
        edges[f].push((t, w));
    });
    edges.iter().map(|e_| {
        e_.sort_by_key(|(t, w)| -w)
    });
    
    let uf = UnionFind::new(nodes.len());
    for v in nodes.iter() {
        uf.alloc(v);
    }

    loop {
        let mut updates = vec!();
        
    }

    mst
}
*/

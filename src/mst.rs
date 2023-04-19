use crate::graph::{Graph,GraphErr};

fn kruskal<G: Graph<usize>> (mut g: G) {
    let nodes = G.get_nodes();
    let edges = G.get_edges();
}


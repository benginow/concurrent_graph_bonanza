pub mod graph;
// pub mod simple;
pub mod coarse;
// pub mod sssp;
// pub mod one;
pub mod tests;

use graph::Graph;

fn main() {
    crate::tests::tcc();
}

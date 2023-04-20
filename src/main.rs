pub mod graph;
// pub mod simple;
pub mod coarse;
// pub mod sssp;
// pub mod one;
pub mod tests;
pub mod t0;

use graph::Graph;

fn main() {
    crate::tests::tcc();
}

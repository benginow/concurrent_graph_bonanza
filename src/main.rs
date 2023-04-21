pub mod graph;

pub mod simple;
// pub mod coarse;
pub mod one;
pub mod csr;
pub mod tests;
pub mod simple_coarse;

use graph::Graph;

fn main() {
    crate::bench::bench();
}

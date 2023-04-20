use std::time::Instant;

pub fn bench() {
    let before = Instant::now();
    crate::benchmark::test();
    println!("Elapsed time: {:.2?}", before.elapsed());
}

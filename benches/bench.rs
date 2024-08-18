use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rusty_algo::sorting::Sortable;

pub fn sorting_benchmark(c: &mut Criterion) {
    let unsorted_vec = (0..1000).rev().collect::<Vec<i32>>(); // Generate a reverse-sorted vector

    // Benchmark for insertion sort
    c.bench_function("insertion_sort 1000", |b| {
        b.iter(|| {
            let mut arr = unsorted_vec.clone(); // Clone the unsorted vector each time
            arr.insertion_sort();
            black_box(arr); // Prevents the compiler from optimizing the sorting away
        });
    });
}

criterion_group!(benches, sorting_benchmark);
criterion_main!(benches);

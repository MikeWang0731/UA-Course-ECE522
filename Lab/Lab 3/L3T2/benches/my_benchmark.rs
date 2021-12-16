use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;

fn selection_sort(arr: &mut [i64]) {
    let len = arr.len();
    // Rust would skip iteration if lower bound >= upper bound.
    // Hence, no need to `len - 1`.
    for i in 0..len {
        let mut temp = i;
        for j in (i + 1)..len {
            if arr[temp] > arr[j] {
                temp = j;
            }
        }
        arr.swap(i, temp);
    }
}

// min_by_key to find the minimum's index and the method swap defined in slices
fn selection_sort_optimize(array: &mut [i64]) {
    for i in 0..array.len() {
        if let Some((j, _)) = array.iter()
            .enumerate()
            .skip(i)
            .min_by_key(|x| x.1) {
            array.swap(i, j);
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut l: Vec<i64> = (0..10000).map(|_| {rng.gen_range(1, 10000)}).collect();
    c.bench_function("selection_sort", |b| b.iter(|| selection_sort(black_box(&mut l))));
    c.bench_function("selection_sort_optimize", |b| b.iter(|| selection_sort_optimize(black_box(&mut l))));

}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
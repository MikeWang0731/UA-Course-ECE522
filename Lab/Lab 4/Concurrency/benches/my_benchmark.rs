use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rayon::prelude::*;


struct Person {
    age: u32,
}

fn age_average() {
    let mut v: Vec<Person> = Vec::new(); for i in 1..1000000 {
        v.push(Person { age: i });
    }
    let num_over_30 = v.iter().filter(|&x| x.age > 30).count() as f32;
    let sum_over_30: u32 = v.iter().map(|x| x.age).filter(|&x| x > 30).sum();
    let avg_over_30 = sum_over_30 as f32/ num_over_30;
}

fn age_average_rayon() {
    let mut v: Vec<Person> = Vec::new(); for i in 1..1000000 {
        v.push(Person { age: i });
    }
    let num_over_30 = v.par_iter().filter(|&x| x.age > 30).count() as f32;
    let sum_over_30: u32 = v.par_iter().map(|x| x.age).filter(|&x| x > 30).sum();
    let avg_over_30 = sum_over_30 as f32/ num_over_30;
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Original", |b| b.iter(|| age_average()));
    c.bench_function("Rayon", |b| b.iter(|| age_average_rayon()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use the_tree::avl_tree::{Avl,Node};

fn test_avl(tree_size: i32) {
    let mut root:Node<_> = Avl::generate_new_tree();
    for i in 0..tree_size {
        root.insert_node(i);
    }
    let search_threshold = 10 / tree_size;
    for i in 0..search_threshold {
        root.is_exist(i);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("AVL TREE", |b| b.iter(|| test_avl(black_box(10000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
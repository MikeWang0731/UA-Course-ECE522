use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use ECE522_project::AVL::{AvlTree, AvlTreeNode};
use ECE522_project::RBTree::RBTree;
use ECE522_project::BST::{BinarySearchTree, Node};

pub fn test_rb_tree(n: i32) {
    let mut rb_tree = RBTree::new();
    for i in 0..n {
        rb_tree.insert_node(i as u32);
    }
    let end = n / 10;
    for j in 0..end {
        rb_tree.search_node(j as u32);
    }
}

pub fn test_avl_tree(tree_size: i32) {
    let mut avl_tree: AvlTreeNode<_> = AvlTree::generate_empty_tree();
    for i in 0..tree_size {
        avl_tree.insert_node(i);
    }
    let end = tree_size / 10;
    for j in 0..end {
        avl_tree.exist_or_not(j);
    }
}

// Binary Search Tree as a basement
pub fn test_BST(tree_size: i32) {
    let mut bs_tree: Node<_> = Node::gen_empty_tree();
    for i in 0..tree_size {
        bs_tree.insert_node(i);
    }
    let end = tree_size / 10;
    for j in 0..end {
        bs_tree.search_node(j);
        // bs_tree.search_node_optimize(j);
    }
}

fn criterion_benchmark_rbtree(c: &mut Criterion) {
    let mut group = c.benchmark_group("rbtree");
    for size in [10000, 40000, 70000, 100000, 130000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                test_rb_tree(size);
            }
            )
        },
        );
    }
    group.finish();
}

fn criterion_benchmark_avltree(c: &mut Criterion) {
    let mut group = c.benchmark_group("avltree");
    for size in [10000, 40000, 70000, 100000, 130000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                test_avl_tree(size);
            }
            )
        },
        );
    }
    group.finish();
}

fn criterion_benchmark_bstree(c: &mut Criterion) {
    let mut group = c.benchmark_group("bstree");
    for size in [10000, 40000, 70000, 100000, 130000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                test_BST(size);
            }
            )
        },
        );
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark_avltree,criterion_benchmark_rbtree,criterion_benchmark_bstree);
criterion_main!(benches);

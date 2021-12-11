fn main() {
    let the_bag = Bag {
        list: [1, 2, 3],
    };

    println!("{:?}", the_bag)
}

#[derive(Debug)]
struct Bag<T> {
    list: T,
}

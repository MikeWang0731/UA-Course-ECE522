struct Bag<T> {
    items: [T; 3],
}

impl<T> Bag<T> {
    fn BagSize(&self) -> usize {
        let mut length = 0;
        for elem in self.items.iter() {
            length = length+std::mem::size_of_val(elem);
        }
        length
    }
}

fn main() {
    let b1 = Bag {
        items: [1u8, 2u8, 3u8],
    };
    let b2 = Bag {
        items: [1u32, 2u32, 3u32],
    };

    println!("size of First Bag = {} bytes", b1.BagSize());
    println!("size of Second Bag = {} bytes", b2.BagSize());
}

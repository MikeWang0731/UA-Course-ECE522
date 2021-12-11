fn main() {

    let b1 = u8_bag{
        items:[1u8,2u8,3u8],
    };

    println!("size of First Bag = {} bytes",b1.get_length());

    let b2 = u32_bag{
        items:[1u32, 2u32, 3u32],
    };

    println!("size of Second Bag = {} bytes",b2.get_length());
}

struct u8_bag{
    items:[u8;3],
}

impl u8_bag {
    fn get_length(&self) -> usize {
        let mut length = 0;
        for elem in self.items.iter() {
            length = length + std::mem::size_of_val(elem);
        }
        length
    }
}

struct u32_bag{
    items:[u32;3],
}

impl u32_bag {
    fn get_length(&self) -> usize {
        let mut length = 0;
        for elem in self.items.iter() {
            length = length + std::mem::size_of_val(elem);
        }
        length
    }
}

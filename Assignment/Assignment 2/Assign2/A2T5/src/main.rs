fn main() {
    let vec1 = vec![12, 32, 13];
    let vec2 = vec![44, 55, 16];
    {
        let vec1_iter = Box::new(vec1.iter()) ;
        println!("vec1_iter after box: {}",std::mem::size_of_val(&vec1_iter));
    }
    {
        let vec_chained = Box::new(vec1.iter()).chain(Box::new(vec2.iter()));
        println!("vec_chained after box: {}",std::mem::size_of_val(&vec_chained));
    }
    {
        let vec1_2 = vec![vec1, vec2];
        let vec_flattened = Box::new(vec1_2.iter()).flatten();
        println!("vec_flattened after box: {}",std::mem::size_of_val(&vec_flattened));
    }
}

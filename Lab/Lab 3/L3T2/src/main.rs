fn main() {
    let mut a = [1, 6, 8, 9, 7, 3, 2];
    selection_sort(&mut a);
    println!("{:?}", a);
}

fn selection_sort(arr: &mut [i32;7]) {
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


use rayon::prelude::*;

fn concurrent_quick_sort(v: &mut [usize]) {
    if v.len() > 1 {
        let mut mid = partition(v);
        if mid < v.len() / 2 {
            mid += 1;
        }
        let (left, right) = v.split_at_mut(mid);
        rayon::join(|| concurrent_quick_sort(left),
                    || concurrent_quick_sort(right));
    }
}


fn partition(v: &mut [usize]) -> usize {
    let l = v.len();  // v.len()
    let mut mid = vec![0; l];
    let pivot = v[0];

    let mut less_array = vec![0; l];
    let mut greater_array = vec![0; l];
    let mut equal_array = vec![0; l];

    for i in 0..l {
        mid[i] = v[i];
        if mid[i] < pivot {
            less_array[i] = 1 as usize;
        } else if mid[i] > pivot {
            greater_array[i] = 1 as usize;
        } else if mid[i] == pivot {
            equal_array[i] = 1 as usize;
        }
    }

    less_array = parallel_prefix_sum(&mut less_array);
    greater_array = parallel_prefix_sum(&mut greater_array);
    equal_array = parallel_prefix_sum(&mut equal_array);

    for i in 0..l {
        if mid[i] < pivot {
            v[less_array[i] - 1] = mid[i];
        } else if mid[i] > pivot {
            v[less_array[l - 1] + equal_array[l - 1] + greater_array[i] - 1] = mid[i];
        } else if mid[i] == pivot {
            v[less_array[l - 1] + equal_array[l - 1] - 1] = mid[i];
        }
    }

    return less_array[l - 1] as usize;
}

fn parallel_prefix_sum(v: &mut [usize]) -> Vec<usize> {
    let mut temp_vec = Vec::new();
    for i in 0..v.len() {
        let mut sum = 0;
        for j in 0..i + 1 {
            sum = sum + v[j];
        }
        temp_vec.push(sum);
    }
    temp_vec
}

fn main() {
    let mut arr = vec![5, 1, 0, 6, 2, 4, 9, 3];
    concurrent_quick_sort(&mut arr);
    println!("{:?}", arr);
}
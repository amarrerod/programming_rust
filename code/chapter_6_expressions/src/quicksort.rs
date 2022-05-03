fn partition<T: Ord + std::fmt::Debug + Clone>(slice: &mut [T], low: isize, high: isize) -> isize {
    println!("Partitioning slice: {:?}", slice);
    let pivot = high as usize;
    let mut i = low - 1;
    let mut j = high;
    loop {
        i += 1;
        while slice[i as usize] < slice[pivot as usize] {
            i += 1;
        }
        j -= 1;
        while j >= 0 && slice[j as usize] > slice[pivot] {
            j -= 1;
        }
        if i >= j {
            break;
        } else {
            slice.swap(i as usize, j as usize);
        }
    }
    slice.swap(i as usize, pivot as usize);
    i
}

fn __quicksort<T: Ord + std::fmt::Debug + Clone>(slice: &mut [T], low: isize, high: isize) {
    if low < high {
        let p = partition(slice, low, high);
        __quicksort(slice, low, p - 1);
        __quicksort(slice, p + 1, high);
    }
}

pub fn quicksort(slice: &mut [i32]) {
    let len = slice.len();
    if len <= 0 {
        return;
    }
    __quicksort(slice, 0, (len - 1) as isize);
}

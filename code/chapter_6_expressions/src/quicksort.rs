fn partition<T: Ord + std::fmt::Debug>(slice: &mut [T], start: &usize, end: &usize) -> usize {
    println!("Partitioning slice: {:?}", slice);
    let mut i = start - 1;
    for j in *start..=(end - 1) {
        let element: &T = &slice[j];
        if *element < slice[*end] {
            i += 1;
            slice.swap(i, j);
        }
    }
    i += 1;
    slice.swap(i, *end);
    println!("Partitioning result: {:?}", slice);
    i
}

pub fn quicksort(slice: &mut [i32], start: &usize, end: &usize) {
    println!("Running quicksort on vector: {:?}", slice);
    if start < end {
        let pivot: usize = partition::<i32>(slice, start, end);
        quicksort(&mut slice[..pivot], start, &(pivot - 1));
        quicksort(&mut slice[pivot + 1..], &(pivot + 1), end);
    } else {
        return;
    }
}

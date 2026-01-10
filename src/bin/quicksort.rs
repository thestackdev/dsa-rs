fn quicksort<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = patition(arr);

    quicksort(&mut arr[..pivot_index]);
    quicksort(&mut arr[pivot_index + 1..]);
}

fn patition<T: Ord + Clone>(arr: &mut [T]) -> usize {
    let pivot_index = arr.len() - 1;

    let mut i = 0;

    for j in 0..arr.len() {
        if arr[j] < arr[pivot_index] {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, pivot_index);

    i
}

fn main() {
    let mut arr = [5, 3634, 3432, 543, 343, 45];
    println!("{:?}", arr);

    quicksort(&mut arr);

    println!("{:?}", arr);
}


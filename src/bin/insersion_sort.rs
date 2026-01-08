fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;

        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

fn main() {
    let mut arr = [5, 3, 6, 2, 6, 0];
    insertion_sort(&mut arr);

    println!("{:?}", arr);
}


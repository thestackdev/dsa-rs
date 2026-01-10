fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let mid = arr.len() / 2;

    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);
    merge(arr, mid);
}

fn merge<T: Ord + Clone>(arr: &mut [T], mid: usize) {
    let left = arr[..mid].to_vec();
    let rigth = arr[mid..].to_vec();

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while left.len() > i && rigth.len() > j {
        if left[i] >= rigth[j] {
            arr[k] = rigth[j].clone();
            j += 1;
        } else {
            arr[k] = left[i].clone();
            i += 1;
        }
        k += 1;
    }

    while left.len() > i {
        arr[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while rigth.len() > j {
        arr[k] = rigth[j].clone();
        j += 1;
        k += 1;
    }
}

fn main() {
    let mut arr = vec![5, 35, 5, 3, 62, 63, 2, 7, 1];

    println!("{:?}", arr);
    merge_sort(&mut arr);
    println!("{:?}", arr);
}


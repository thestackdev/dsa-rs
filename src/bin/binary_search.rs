use std::cmp;

pub fn binary_search<T: Ord>(array: &[T], search_key: T) -> Result<usize, usize> {
    let mut start: usize = 0;
    let mut end: usize = array.len();

    while start < end {
        let middle = start + (end - start) / 2;

        match array[middle].cmp(&search_key) {
            cmp::Ordering::Less => start = middle + 1,
            cmp::Ordering::Greater => end = middle,
            cmp::Ordering::Equal => return Ok(middle),
        }
    }

    Err(start)
}

fn main() {}

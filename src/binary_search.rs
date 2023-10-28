use std::cmp::Ordering;


pub fn binary_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    let mut is_asc = true;

    if arr.len() > 1 {
        is_asc = arr[0] < arr[arr.len() - 1];
    }

    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;

        if is_asc {
            match item.cmp(&arr[mid]) {
                Ordering::Less => right = mid,
                Ordering::Equal => return Some(mid),
                Ordering::Greater => left = mid + 1,
            }
        } else {
            match item.cmp(&arr[mid]) {
                Ordering::Less => left = mid + 1,
                Ordering::Equal => return Some(mid),
                Ordering::Greater => right = mid,
            }
        }
    }

    None
}


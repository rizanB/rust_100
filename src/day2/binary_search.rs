// returns index of element if found, -1 if not found
fn binary_search(arr: &[i32], target: i32) -> i32 {
    let mut left = 0;
    let mut right = arr.len() as i32 - 1;

    while left <= right {
        let mid = left + (right - left) / 2;

        // need usize for array indexing
        let mid_usize = mid as usize;

        if arr[mid_usize] == target {
            return mid_usize as i32;
        } else if arr[mid_usize] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    return -1;
}

pub fn main() {
    let arr = vec![1, 3, 5, 7, 9];
    let target = 323;

    let target_index = binary_search(&arr, target);

    match target_index {
        0 => println!("found in first position"),
        1 | 2 | 3 => println!("found in second, third or fourth positions"),
        4..=10 => println!("found in fifth - tenth position"),
        _ => println!("not found within first ten positions"),
    }
}

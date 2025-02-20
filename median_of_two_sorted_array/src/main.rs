use std::vec;
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut array: Vec<i32> = [nums1, nums2].concat();
    array.sort();
    if array.len() == 1 {
        array[0] as f64
    } else if array.len() % 2 == 0 {
        let result = array.len() / 2;
        let result1 = array[result - 1] as f64;
        let result2 = array[result] as f64;
        (result1 + result2) / 2.0
    } else {
        array[array.len() - 2] as f64
    }
}

fn main() {
    let result = find_median_sorted_arrays(
        vec![1, 8, 7, 4, 6, 5, 1, 3],
        vec![9, 8, 7, 4, 6, 5, 1, 6, 9, 8],
    ); // [111344556667788899]
    println!("The median of the sorted arrays is: {}", result);
}

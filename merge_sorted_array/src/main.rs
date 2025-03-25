use core::num;



pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        nums2.truncate(n as usize);

        nums1.truncate(m as usize);
        nums1.extend(nums2.iter());
        nums1.sort();

    }




fn main() {
    println!("Hello, world!");
}

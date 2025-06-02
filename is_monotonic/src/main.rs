


pub fn is_monotonic(nums: Vec<i32>) -> bool {
    let mut is_increasing = true;

    for i in 1..nums.len() {
        if nums[i] <= nums[i - 1] {
            is_increasing = false;  
        }
        if nums[i] > nums[i - 1] {
            is_increasing = false;
        }
    }
    is_increasing
}




fn main() {
    let nums = vec![1, 2, 2, 3];    
    println!("{}", is_monotonic(nums));
}

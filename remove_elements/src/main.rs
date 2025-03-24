fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut count = 0;
    for i in 0..nums.len() - 1 {
        
        if val == nums[i] {
            nums.remove(i);
            count += 1
        }
    }
    println!("The array is: {:?}", nums);
    count
}

fn main() {
    let result = remove_element(&mut vec![0,1,2,2,3,0,4,2], 2);
    println!("{}", result);
}

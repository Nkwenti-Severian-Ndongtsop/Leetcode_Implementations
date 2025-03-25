
fn single_number(mut nums: Vec<i32>) -> i32 {
    // 2, 2, 1
    nums.sort();
    println!("{:?}", nums);

    for i in 0..nums.len() {
        if nums[i] != nums[i+1] {
            return nums[i]
        } else if nums[i] != nums[i+1] && i == nums.len() - 1{
            return nums[i + 1]
        }
    }
0
}

fn main() {
    println!("The single number is: {}", single_number(vec![4,1,2,1,2]));
}

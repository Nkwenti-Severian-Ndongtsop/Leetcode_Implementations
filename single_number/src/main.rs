
fn single_number(mut nums: Vec<i32>) -> i32 {
    for i in 0..nums.len() - 2 {
        for j in i + 1..nums.len() - 1 {
            if nums[i] == nums[j] {
                nums.remove(i);
                nums.remove(j);
                break;
            }
        }
    }
    nums[0]
}

fn main() {
    println!("The single number is: {}", single_number(vec![2, 2, 1]));
}

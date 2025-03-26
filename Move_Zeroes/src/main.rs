pub fn move_zeroes(nums: &mut Vec<i32>) -> &mut Vec<i32> {
    for i in 0..nums.len() {
        if nums[i] == 0 {
            let value = nums.remove(i);
            nums.push(value);
        }
    }

    nums
}

fn main() {
    let mut nums = vec![0, 1, 0, 3, 12];
    println!("{:?}", move_zeroes(&mut nums));
}

fn longest_consecutive(mut nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    nums.sort();
    let mut count = 1;

    for i in 1..nums.len() { // 1  2 3 4 500 41 725
        if nums[i] == nums[i - 1] + 1 { // 1+1
            count += 1;
        } else  {
            continue;
        }
    }
        count
}

fn main() {
    let values: Vec<i32> = vec![1,2,3,4,500,41,725];
    let value = longest_consecutive(values);
    println!("{}", value)
}
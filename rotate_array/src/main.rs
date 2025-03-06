
pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let mut n = nums.len(); //  7
    let mut result: Vec<i32> = vec![];

    for _ in 1..=k {
        let value = nums.remove(n - 1);
        result.insert(0, value);
        n -= 1;
    }
    result.append(nums);
    println!("{:?}", result)
}

fn main() {
    let test: &mut Vec<i32> = &mut vec![7, 1, 2, 3, 4, 5, 6];
    let k = 2;
    println!("The output is {:?}", rotate(test, k));
}



pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut result = nums.iter().map(|num| num.pow(2)).collect::<Vec<i32>>();
    result.sort();
    result
}



fn main() {
    let nums = vec![-4,-1,0,3,10];
    println!("The sorted square is: {:?}", sorted_squares(nums));
}

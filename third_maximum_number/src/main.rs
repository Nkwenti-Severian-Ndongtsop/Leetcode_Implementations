pub fn third_max(mut nums: Vec<i32>) -> i32 {
    nums.sort();
    let mut limit = 3;
    let mut index = 0;
    let mut max: Vec<i32> = vec![nums[nums.len() - 1]];
    let mut k = 0;
    
    while limit > 0 {
        let mut temp_max = 0;
        
        for i in 0..nums.len() {
            if nums[i] > temp_max && nums[i] != max[k]{ 
                temp_max = nums[i];
                index = i;
            }
        }
        println!("{temp_max}");
        nums.remove(index);
        max.push(temp_max);
        println!("{:?}", max);
        println!("{:?}", nums);
        limit -= 1;
        k += 1
    }
    max[0]
}

fn main() {

    let nums = vec![2,2,3,1]; // 
    println!("{}", third_max(nums));

}

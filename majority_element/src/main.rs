
fn majority_element(mut values: Vec<u32>) -> u32 {
    values.sort();
    return values[values.len() / 2]
}
fn main() {

    println!("The majority element is: {}", majority_element(vec![2,2,1,1,1,2,2]));
}

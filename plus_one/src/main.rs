use std::vec;


fn plus_one(mut digits: Vec<u32>) -> Vec<u32> {

    // let last = values.len() - 1;
    let last_values = digits.remove(digits.len() - 1);
    digits.push(last_values + 1);
    digits


}




fn main() {
    println!("The plus_one is: {:?}", plus_one(vec![4, 3, 2, 1]));
}

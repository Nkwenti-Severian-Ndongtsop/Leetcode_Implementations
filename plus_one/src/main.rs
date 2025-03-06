use std::vec;


fn plus_one(mut values: Vec<u32>) -> u32 {

    // let last = values.len() - 1;
    let last_values = values.remove(values.len() - 1);
    values.push(last_values + 1);
    let result = values.iter().map(|&value| value.to_string()).collect::<String>().parse().expect("Couldn't collect the value");
    result


}




fn main() {
    println!("The plus_one is: {}", plus_one(vec![4, 3, 2, 1]));
}


pub fn multiply_string(num1: &str, num2: &str) -> String {
    
    let first_value = num1.parse::<u64>().expect("Couldn't get value");
    let second_value = num2.parse::<u64>().expect("Couldn't get value");

    let result = (first_value * second_value).to_string();
    result
    
}



fn main() {
    
    let values = ("123", "456");
    println!("The product of this strings is: {}", multiply_string(values.0, values.1));
}

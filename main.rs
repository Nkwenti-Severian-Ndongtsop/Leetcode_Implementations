use std::io;

fn main() {

    let mut first_value = String::new();
   
    let mut second_value = String::new();
   
    println!("Input the first value");

    io::stdin()
    .read_line(&mut first_value)
    .expect("Could not read the first value");

    println!("Input the second value");

    io::stdin()
    .read_line(&mut second_value)
    .expect("Could not read the second value");

let  first_value: u8 = first_value.trim().parse().expect("couldn't read the value");

let  second_value: u8 = second_value.trim().parse().expect("couldn't read the value");

let result: u8 = {first_value} + {second_value};

println!("The sum is {result}");

}
use std::collections::HashMap;

fn single_number(mut values: Vec<u32>) -> u32 {
    for i in 0..values.len() - 2 {
        for j in i + 1..values.len() - 1 {
            if values[i] == values[j] {
                values.remove(i);
                values.remove(j);
                break;
            }
        }
    }
    values[0]
}

fn main() {
    println!("The single number is: {}", single_number(vec![2, 2, 1]));
}

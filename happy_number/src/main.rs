use std::collections::HashSet;

pub fn is_happy(mut n: i32) -> bool {
    let mut seen = HashSet::new();

    while n != 1 && seen.insert(n) {
        n = n
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap().pow(2))
            .sum::<u32>() as i32;
    }

    n == 1
}

fn main() {
    let test = 19;
    let result = is_happy(test);
    println!("{:?}", result);
}
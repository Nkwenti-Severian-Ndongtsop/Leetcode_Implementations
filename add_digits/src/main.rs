

pub fn add_digits(num: i32) -> i32 {
        let mut string_num = num.to_string();
    
        while string_num.len() > 1 {
            let string = string_num.chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>();
           string_num = string.to_string();
        }
        string_num.parse::<i32>().unwrap()      
}






fn main() {
    let num = 38;
    println!("{}", add_digits(num));
}

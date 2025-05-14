fn main() {
    fn palindrome_bool(num1: i32) -> bool {
        // 123
        let mut num1_str: String = num1.abs().to_string().chars().rev().collect();
        let index = num1_str.len();
        if num1 < 0 {
            num1_str.insert(index, '-');
        }
        return num1_str == num1.to_string();
    }
    println!("Is this a palindrome: {}", palindrome_bool(-121));

    }
    
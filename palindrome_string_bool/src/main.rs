
fn palindrom(string: &str) -> bool {

    let filtered: String = string
    .chars()
    .filter(|&s| s.is_alphanumeric())
    .map(|s| s.to_ascii_lowercase()).collect();

    filtered == string
    .chars()
    .filter(|&s| s.is_alphanumeric())
    .map(|s| s.to_ascii_lowercase()).rev().collect::<String>()
}

fn main() {
    let word = "A man, a plan, a canal: Panama";
    println!("Is the word {word} a palindrome: {}", palindrom(word));
}

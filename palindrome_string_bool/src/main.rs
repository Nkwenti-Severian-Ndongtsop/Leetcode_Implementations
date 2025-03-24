
fn palindrom(s: &str) -> bool {

    let filtered: String = s
    .chars()
    .filter(|&s| s.is_alphanumeric())
    .map(|s| s.to_ascii_lowercase()).collect();

    filtered == s
    .chars()
    .filter(|&s| s.is_alphanumeric())
    .map(|s| s.to_ascii_lowercase()).rev().collect::<String>()
}

fn main() {
    let word = "A man, a plan, a canal: Panama";
    println!("Is the word {word} a palindrome: {}", palindrom(word));
}

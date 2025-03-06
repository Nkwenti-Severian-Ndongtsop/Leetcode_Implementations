
fn last_word(s: String) -> i32 {
    let string: Vec<String> = s.split_whitespace().rev().map(String::from).collect();
    string[0].len() as i32
   
}
fn main() {
    let string = String::from("Nkwenti Severian");
    println!("{}", last_word(string));
}

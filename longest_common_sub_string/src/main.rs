// fn longest_common_prefix(strs: Vec<&str>) -> String {
//     if strs.is_empty() { return String::new(); }
//     let first = &strs[0];
//     for (i, c) in first.chars().enumerate() {
//         for s in &strs {
//             if s.chars().nth(i) != Some(c) {
//                 return first[..i].to_string();
//             }
//         }
//     }
//     first.to_string()
// }

fn multiply(num1: String, num2: String) -> String {
        let num1 = num1.trim().parse();
        let num2 = num2.trim().parse();
        let result = num1 * num2 ;
        return result.to_string() ;
    }
fn main() {
    // let strs = vec!["flower", "flow", "flight"];
    // println!("Longest common prefix: {}", longest_common_prefix(strs));
  println!("{}", multiply("123", "155"))

}



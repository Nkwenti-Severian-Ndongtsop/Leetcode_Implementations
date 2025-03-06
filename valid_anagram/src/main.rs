fn valid_anagram(string1: &str, string2: &str) -> bool {

    // "rat"    "car"

    let mut new_string1: Vec<char> = string1.chars().collect();
    let mut new_string2: Vec<char> = string2.chars().collect();

    for i in 0..new_string1.len() - 1 {
        
        for j in i..new_string2.len() - 1 {
            
            if new_string1[i] == new_string2[j] {
                new_string1.remove(i);
                new_string2.remove(j);
            }
        }
    }
    if new_string1.len() == new_string2.len() {
        return true
    } else {
        false
    }
}



fn main() {

    let first_string = "rat";
    let second_string = "car";
    println!("Is the word {first_string} and {second_string} an anagram:  {}", valid_anagram(first_string, second_string));
}

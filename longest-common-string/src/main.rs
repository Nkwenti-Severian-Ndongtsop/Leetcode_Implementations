use std::env;

use utils::longest_common_substring;


fn main() {
    let args: Vec<String> = env::args().collect();
    let string = &args[1];
    longest_common_substring(string.to_string());
}

mod utils;
fn power_of_two(n: u32) -> bool {

    if n == 1 {
        return true;
    }

    let mut check = 1;

    while check <= n {
        if check == n {
            return true
        }
        check = check * 2;
    }
false
}

fn main() {
    let num = 1073741825;
    println!("Is the value {num} a multiple of 2: {}", power_of_two(num));
}

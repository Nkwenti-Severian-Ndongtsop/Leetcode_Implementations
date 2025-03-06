fn power_of_two(num: u32) -> bool {

    if num == 1 {
        return true;
    }

    let mut check = 1;

    while check <= num {
        if check == num {
            return true
        }
        check = check * 2;
    }
false
}

fn main() {
    let num = 5;
    println!("Is the value {num} a multiple of 2: {}", power_of_two(num));
}

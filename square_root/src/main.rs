
fn square_root(num: u32) -> u32 {

    let mut guess = 0;
     
     while (guess + 1) * (guess + 1) <= num {
        guess += 1
     }
     guess
}



fn main() {
    println!("The approxiamted square_root is: {}", square_root(25))
}




fn climbing_stairs(num: u32) -> u32 {

    if num == 2 {
        return 2
    }
    fn factorial(n: u32) -> u32 {
        let mut result = 1;
        if n == 0 || n == 1 {
            return 1
        }
        for i in 1..=n {
            result = result * i
        }
        result
    }

    let result = factorial(num) / (factorial(2) * factorial(num - 2));
    result
}



fn main() {
    let val = 3;
    println!("The climbing_stairs of {} is: {}", val, climbing_stairs(val));
}

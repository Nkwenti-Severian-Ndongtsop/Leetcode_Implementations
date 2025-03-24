


fn climbing_stairs(n: i32) -> i32 {
    if n == 2 {
        return 2
    }
    if n==1 {
        return 1
    }
    fn factorial(n: i32) -> i32 {
        let mut result = 1;
        if n == 0 || n == 1 {
            return 1
        }
        for i in 2..=n {
            result = result * i
        }
        result
    }

    let result = factorial(n) / (factorial(2) * factorial(n - 2));
    result
}



fn main() {
    let val = 4;
    println!("The climbing_stairs of {} is: {}", val, climbing_stairs(val));
}

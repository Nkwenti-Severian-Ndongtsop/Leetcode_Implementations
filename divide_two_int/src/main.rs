fn main() {
//    let  result = num1 / num2;
//     println!("The result is: {}", result)
// }


// fn main() {
    
//     let mut dividend = String::new();
//     println!("Input the dividend");
//     io::stdin()
//     .read_line(&mut dividend)
//     .expect("Invalid dividend");

//     let mut divisor = String::new();
//     println!("Input the divisor");
//     io::stdin()
//     .read_line(&mut divisor)
//     .expect("Invalid divisor");

//     let dividend: f64 = dividend.trim().parse().expect("could not collect dividend");

//     let divisor: f64 = divisor.trim().parse().expect("could not collect divisor");

//     divide( dividend, divisor);
// }

    fn divide(dividend: i32, divisor: i32) -> i32{
        
    return dividend / divisor ;
}
    divide(10, 3);
}

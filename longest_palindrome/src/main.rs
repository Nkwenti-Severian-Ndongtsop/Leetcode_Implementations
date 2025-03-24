pub fn multiply_string(num1: String, num2: String) -> String {
    if num1 == "0" || num2 == "0" {
        return "0".to_string();
    }

    let mut result = vec![0; num1.len() + num2.len()];
    let num1_bytes = num1.as_bytes();
    let num2_bytes = num2.as_bytes();

    for i in (0..num1.len()).rev() {
        for j in (0..num2.len()).rev() {
            let mul = (num1_bytes[i] - b'0') as usize * (num2_bytes[j] - b'0') as usize;
            let sum = mul + result[i + j + 1];

            result[i + j + 1] = sum % 10;
            result[i + j] += sum / 10;
        }
    }

    let mut result_str = String::new();
    let mut leading_zero = true;

    for &digit in &result {
        if digit != 0 || !leading_zero {
            leading_zero = false;
            result_str.push((digit as u8 + b'0') as char);
        }
    }

    result_str
}

fn main() {
    let values = (
        "401716832807512840963".to_string(),
        "167141802233061013023557397451289113296441069".to_string(),
    );
    println!(
        "The product of this strings is: {}",
        multiply_string(values.0, values.1)
    );
}

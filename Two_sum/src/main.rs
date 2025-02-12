fn main() {
    let array = [2, 7, 11, 15];
    let target = 9;
    let mut count = 0;
    let mut result: [usize; 2] = [0, 0];
    loop {
       if count >= array.len() {
        break;
       } else if array[count] + array[count + 1] == target{
        result =  [count, count + 1];
        break;
       }
        count += 1;
    }
        println!("The index of target is {:?}", result)

       }

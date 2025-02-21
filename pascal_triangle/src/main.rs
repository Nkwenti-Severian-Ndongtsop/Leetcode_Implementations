fn pascal_triangle(pos: u32) -> Vec<u32> {
    let mut prev_cell: Vec<u32> = vec![1]; // first element always having 1

    for _ in 0..pos {
        // iterate over the rows based on the pos
        let next_length = prev_cell.len() + 1; // increment the length of the next row
        let mut curr_cell = vec![0; next_length]; //create a new vector based on the previous + 1
        for j in 0..next_length {
            // loop through the current cell
            if j == 0 || j == next_length - 1 {
                curr_cell[j] = 1;
            } else {
                curr_cell[j] = prev_cell[j] + prev_cell[j - 1];
            }
        }
        prev_cell = curr_cell;
    }
    prev_cell
}

fn main() {
    let value = 5;
    println!("The pascal triangle for {} is: {:?}", value, pascal_triangle(value))
}

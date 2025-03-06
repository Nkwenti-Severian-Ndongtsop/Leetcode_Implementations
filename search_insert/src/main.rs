fn search_insert(values: Vec<i32>, target: i32) -> i32 {
    if !values.is_empty() {
        for i in 0..values.len() - 1 {
             if values[i] >= target {
                return i as i32;
            }
        }
    }
    values.len() as i32
}

fn main() {
    let values = vec![1, 3, 5, 6];
    let target = 4;

    println!("The output is: {}", search_insert(values, target))
}

#[cfg(test)]

mod test {
    use crate::search_insert;


    #[test]

    fn test_search_insert() {
        let values = vec![1, 3, 5, 6];
    let target = 4;
        assert_eq!(search_insert(values, target), 2);
    }
}

pub fn longest_common_substring(string: String) -> String {
    if string.len() < 2 {
        return string;
    }

    let mut arr_slices: Vec<String> = Vec::new();

    // Hello world
    // ["Hel", "el", "l", "lo W", "o w", " world", "world", ...]
    // expected Rusult:
    for i in 0..string.len() {
        let current_char = match string.get(i..i+1) {
            Some(char) => char,
            None => panic!("Index Out of band error"),
        };

        // }
        // let mut char_iter = string.chars();
        // while let Some(current_char) = char_iter.next() {
        // let arr_slices: &Vec<char> =  &arr_slices;

        // let mut arr_slices_iter = arr_slices.into_iter();
        // while let Some(char) = arr_slices_iter.next()  {

        // }

        arr_slices.push(current_char.to_string());
        println!("arr: {:?}", arr_slices);

        for j in (i + 1)..string.len() {
            let char = match string.get(j..j+1) {
                Some(char) => char,
                None => panic!("Index Out of band error"),
            };
            println!("{i}. {current_char} <==> {j}. {char}");
            // }

            // let mut char_iter = string.chars().skip(char_iter.);
            // while let Some(char) = char_iter.next() {
            let index = arr_slices.len() - 1;

            match arr_slices.get_mut(index) {
                Some(slice) => {
                    if slice.contains(char) {
                        break;
                    }

                    slice.push_str(char);
                }
                None => {
                    // arr_slices.push(char.to_string());
                    break;
                }
            };
        }
    }

    let (mut slice_len, mut max_slice) = (0, String::new());
    for slice in arr_slices {
        if slice.len() > slice_len {
            slice_len = slice.len();
            max_slice = slice;
        }
    }

    max_slice
}
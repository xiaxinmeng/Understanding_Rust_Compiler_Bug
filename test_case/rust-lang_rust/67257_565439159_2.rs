rust
let sub_cursor_2 = sub_cursor.sub_cursor(0, 3);

let mut result = vec![];
sub_cursor_2.read_to_end(&mut result)?;

assert_eq!(vec![1, 2, 3], result);

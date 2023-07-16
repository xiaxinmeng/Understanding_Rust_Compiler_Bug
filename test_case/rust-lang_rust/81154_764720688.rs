rust
let vec = vec![1, 2, 3];
let range = (4..5).assert_len(3); // returns 3..3
for n in &vec[range] {
    unreachable!("the slice is empty");
}

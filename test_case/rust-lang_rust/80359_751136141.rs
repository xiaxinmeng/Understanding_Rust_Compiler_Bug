rust
pub fn negative_then_positive(input: &[i32]) -> Vec<i32> {
    let mut negative = vec![];
    let mut positive = vec![];
    for x in input {
        match x.cmp(&0) {
            Less => negative.push(*x),
            Equal => panic!("expecting nonzero values"),
            Greater => positive.push(*x),
        }
    }
    negative.extend(positive);
    negative
}

 rust
fn process_123(processor: |&[u8]| -> Vec<u8>) -> Vec<u8> {
    processor(&[1, 2, 3])
}

fn main() {
    let make_vec = |input/*: &_*/| {  // UNCOMMENT THIS TO FIX
        let mut vector = Vec::new();
        vector.push_all(input);
        vector
    };

    assert_eq!(process_123(make_vec), vec![1, 2, 3]);
}

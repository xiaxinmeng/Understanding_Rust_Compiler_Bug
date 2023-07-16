
fn main() {
    let vecs = vec![vec![1], vec![1, 2], vec![1, 2, 3]];
    for len in vecs.iter().map(Vec::len) { // <- No autoderef happens here, len is not found
        println!("{}", len);
    }
}

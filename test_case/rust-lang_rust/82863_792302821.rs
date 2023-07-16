rust
fn main() {
    let be = vec!['a', 'b', 'c'];
    let splitted_vec = be.splitn(3, '-');
    for i in splitted_vec {
        println!("{:?}",i);
    }
}

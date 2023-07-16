 rust
fn main() {
    let v = vec![&3i, &4i, &5i];

    for &&x in v.iter() {
        println!("{}", x + 3);
    }
}


#![feature(slice_patterns)]
fn main() {
    let x: &[String] = &[];
    match x {
        &[[ref b..]..] => { println!("Fun{}{}", b.len()); }
    }
}

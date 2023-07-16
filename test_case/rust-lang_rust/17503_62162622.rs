 rust
#![feature(slicing_syntax)]

fn main() {
    let s: &[int] = &[0, 1, 2, 3, 4];
    let ss: &&[int] = &s;
    let sss: &&&[int] = &ss;

    println!("{}", s[..3]);
    println!("{}", ss[3..]);
    println!("{}", sss[2..4]);
}

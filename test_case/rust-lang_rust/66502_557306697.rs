rust
#![allow(unused)]

fn main() {
    let a = [(String::from("HELLO"), String::from("WORLD"))];
    let [mut x] = a;
    x = (String::from("GOODBYE"), String::from("RUST"));
    let [(ref y, ref z)] = a;
    println!("{:?}", (y, z));
}

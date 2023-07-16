 rust
#![feature(box_syntax)]
#![allow(unstable)]
fn main(){
    let celebrate: Box<Fn(i64)> = box |&: num: i64| {
        println!("Feels great to implement C++ std::function in {} lines.", num)
    };
    let lucky_number: Box<Fn(i64)> = box |&: num: i64| {
        println!("My lucky number is {}", num)
    };
    let func = if std::rand::random() { celebrate } else { lucky_number };
    func(25)
}

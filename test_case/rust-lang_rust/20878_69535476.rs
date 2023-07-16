 Rust
#![feature(unboxed_closures, box_syntax)]
#![allow(unstable)]

fn anyfunc<'a, Args, Ret, F>(closure: F) -> Box<Fn<Args, Ret> + 'a>
where F: Fn<Args, Ret> + 'a {
    box closure
}

fn main(){
    let celebrate = anyfunc(|&: num: i64| {
        println!("Feels great to implement C++ std::function in {} lines.", num)
    });
    let lucy_number = anyfunc(|&: num: i64| {
        println!("My lucky number is {}", num)
    });
    let func = if std::rand::random() { celebrate } else { lucy_number };
    func(25)
}

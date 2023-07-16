rust
#![feature(get_mut_unchecked)]

use std::rc::Rc;

unsafe fn f<'a>(mut r: Rc<&'a str>, s: &'a str) {
    *Rc::get_mut_unchecked(&mut r) = &s;
}

fn main() {
    let x = Rc::new("Hello, world!");
    {
        let s = String::from("Replaced");
        unsafe { f(x.clone(), &s) };
    }
    println!("{}", x);
}

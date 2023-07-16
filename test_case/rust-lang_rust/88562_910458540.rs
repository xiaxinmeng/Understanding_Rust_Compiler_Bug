rs
fn main() {
    let x = Delegate(foo);
    println!("{:?}", x);
}

fn foo(_: &String) {}

pub struct Delegate<T> (fn(Data: &T));

use std::fmt;

impl<T> fmt::Debug for Delegate<T> {
    fn fmt<'a, 'b>(&'a self, f: &mut fmt::Formatter<'b>) -> std::fmt::Result {
        f.debug_tuple("Delegate").field(&self.0 as &fn(&'a T)).finish()
    }
}

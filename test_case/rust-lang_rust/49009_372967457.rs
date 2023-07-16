rust
use std::boxed::Box;

/// compile ok
fn Box<T>(t:T) -> Box<T> {
    Box::new(t)
}

/// compile error:
///    |
/// 11 | struct MyFoo<T>(T);
/// | ------------------- previous definition of the value `MyFoo` here
/// ...
/// 18 | fn MyFoo<T>(t:T) -> MyFoo<T>{
///    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `MyFoo` redefined here
///        |
///        = note: `MyFoo` must be defined only once in the value namespace of this module
struct MyFoo<T>(T);
impl<T> MyFoo<T> {
    fn new(t:T) -> Self {
        MyFoo(t)
    }
}

fn MyFoo<T>(t:T) -> MyFoo<T>{
    MyFoo::new(t)
}

fn main(){

}

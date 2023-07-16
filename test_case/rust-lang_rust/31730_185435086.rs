 rust
trait FooFn {
    fn foo(&self);
}

impl FooFn for for<'a> fn(&'a mut i32)-> i32 {
    fn foo(&self) {
        println!("Generic (with mutable) match!");
    }
}

impl<'a> FooFn for fn(&'a mut i32)-> i32 {
    fn foo(&self) {
        println!("Generic lifetime match!");
    }
}

impl FooFn for fn(i32)->i32 {
    fn foo(&self) {
        println!("Generic match!");

    }    
}

fn simple_fn(x: i32) -> i32 { 1 }
fn simple_fn2(x: &mut i32) -> i32 { 2 }

fn strip(x: fn(i32)->i32) -> fn(i32)->i32 { x }
fn strip2(x: fn(&mut i32)->i32) -> fn(&mut i32)->i32 { x }
fn strip3<'a>(x: fn(&'a mut i32)->i32) -> fn(&'a mut i32)->i32 { x }
fn strip4(x: fn(&'static mut i32)->i32) -> fn(&'static mut i32)->i32 { x }

fn main() {
    &(simple_fn as fn(i32)->i32).foo(); 
    &(simple_fn2 as fn(&mut i32)->i32).foo();

    strip(simple_fn).foo();
    strip2(simple_fn2).foo();
    strip3(simple_fn2).foo();
    strip4(simple_fn2).foo();
}

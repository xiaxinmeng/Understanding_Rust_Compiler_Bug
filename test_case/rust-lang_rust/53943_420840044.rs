rust
trait MyF<'a>: FnOnce() -> &'a () {}

fn test<F>(_: F) where for<'a> F: MyF<'a, Output=&'a ()> {
}

fn main(){
}

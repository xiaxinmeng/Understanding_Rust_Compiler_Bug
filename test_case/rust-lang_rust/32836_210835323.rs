 rust
> fn main() {
>     let mut x: (Box<i32>, i32);
>     x.0 = Box::new(2); // x.0 statically know to be uninit, destructor not called
>     x.0 = Box::new(3); // x.0 destructor is called
> }
> 
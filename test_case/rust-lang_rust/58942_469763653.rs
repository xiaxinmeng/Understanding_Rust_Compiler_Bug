rust
struct X;

impl X {
   fn f() {}
}

macro_rules! do_impl { () => {
    impl X {
       fn f() {}
    }
}}
do_impl!();

fn main() {}

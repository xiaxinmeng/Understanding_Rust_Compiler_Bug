 rust
// foo.rs
#![crate_type = "lib"]
#![crate_name = "foo"]
pub trait Future {
    type Item;
    type Error;
}

fn foo() -> Box<Future<Item=(), Error=Box<()>>> {
    loop {}
}

pub fn bar<F, A, B>(_s: F)
    where F: Fn(A) -> B,
{
    foo();
}

// bar.rs
extern crate foo;

fn mk<T>() -> T { loop {} }

struct Data<T, E> {
    data: T,
    error: E,
}

fn main() {
    foo::bar(|()| {
        Data::<(), std::io::Error> {
            data: mk(),
            error: mk(),
        }
    })
}

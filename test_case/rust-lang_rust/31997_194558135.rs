 rust
trait TheTrait { }

fn closure<F, T>(x: F) -> Result<T, ()>
    where F: FnMut() -> T, T: TheTrait,
{
    unimplemented!()
}

fn foo() {
    try!(closure(|| bar(0 as *mut _))) // bar intentionally undefined
}

fn main() { }

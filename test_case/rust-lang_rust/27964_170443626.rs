 rust
trait Reader{}
impl Reader for () {}
fn reader_for()-> Box<Reader> { Box::new(()) }

fn check_vec<R:Reader>(r:&R){} //R needs to be Reader+?Sized for some reason, but new rust programmers wont know that.

fn main() {
    check_vec(&*reader_for());
        //error: the trait `core::marker::Sized` is not implemented for the type `Reader`
        //note: `Reader` does not have a constant size known at compile-time
        //note: required by `check_vec`
}

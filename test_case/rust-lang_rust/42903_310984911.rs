rust
pub struct DropHasLifetime<'a>(&'a ());

impl<'a> ::std::ops::Drop for DropHasLifetime<'a> {
    fn drop(&mut self) {}
}

fn move_and_return<T>(val: T) -> T {
    val
}

struct Wrapper<'a>(DropHasLifetime<'a>);

fn main() {
    static STATIC: () = ();

    let mut wrapper = Wrapper(DropHasLifetime(&STATIC));

    wrapper.0 = move_and_return(wrapper.0);
}

rust
pub fn bar<'a>(this: &'a mut ((), &'static ())) {
    let beta = from(&this.0);
    let locked = as_ref(&beta);
    write(&mut this.1, locked);
}

fn write<T>(_: *mut T, _: T) {}
fn from<'a>(_: &'a ()) -> &'static () { panic!() }
fn as_ref<'a>(_: &'a &'static ()) -> &'a () { panic!() }

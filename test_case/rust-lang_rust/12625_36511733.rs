
struct RcBox<T> {
    value: T,
    strong: Cell<uint>,
    weak: Cell<uint>
}

pub struct Rc<T> {
    priv ptr: *mut RcBox<T>,
}

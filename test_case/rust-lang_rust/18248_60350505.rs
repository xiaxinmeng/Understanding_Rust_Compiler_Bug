
struct RcBox<Sized? T> {
    strong: Cell<uint>,
    weak: Cell<uint>,
    value T
}

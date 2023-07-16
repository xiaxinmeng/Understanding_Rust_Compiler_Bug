rust
fn want_send<T: Send>(t: T) { }
fn want_sync<T: Sync>(t: T) { }

// move will not help because `x` is itself a reference:
fn foo(x: &Cell<T>) {
    want_send(|| x.set(1));
}

// move will not help because `Cell<T>` is not sync:
fn foo(x: Cell<T>) {
    want_sync(|| x.set(1));
}

 rust
fn foo(..., _ret: &mut IoError<T>);

fn bar(..., _ret: &mut IoError<T>) {
    foo(..., _ret);
    if !_ret.is_ok() { return; } // no need to copy

    // ...

    // write our own value when we're done
    *_ret = ...;
}

fn baz() {
     let mut _ret: IoError<T> = uninitialized();
     bar(..., &mut _ret);
}

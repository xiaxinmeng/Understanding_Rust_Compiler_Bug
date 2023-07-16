rust
fn f<T: Sized>() {}
fn g() { f::<str>() }
fn h() { f::<str>() }

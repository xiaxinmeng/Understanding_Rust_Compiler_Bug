rust
/// Allowed:
fn erase0<T: 'static + for<'a> As<'a>>() {
    assert_static::< fn(T) >();
    assert_static::< fn(&T) >();
    assert_static::< for<'a> fn(<T as As<'a>>::Out) >();
}

/// Not allowed:
fn erase1<'a>() { assert_static::< fn(&'a i32) >(); }
fn erase2<T>() { assert_static::< fn(T) >(); }
fn erase3<T: for<'a> As<'a>>() {
    assert_static::< for<'a> fn(<T as As<'a>>::Out) >();
}

/// Type-level function to replace a type's lifetime parameter.
pub trait As<'a> {
    type Out: 'a;
}

/// Ex: convert &'a T to &'b T.
impl<'a, 'b, T: 'b> As<'b> for &'a T {
    type Out = &'b T;
}

fn assert_static<T: 'static>() {}

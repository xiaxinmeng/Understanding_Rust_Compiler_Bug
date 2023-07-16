rust
fn f() -> impl for<'a> Trait<'a, Out = impl Sized> { ... }
fn f<'b>() -> impl for<'a> Trait<'a, Out = impl Sized + 'b> { ... }
fn f() -> impl for<'a> Trait<'a, Out = impl Sized + 'static> { ... }

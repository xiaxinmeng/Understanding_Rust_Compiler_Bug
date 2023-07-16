rust
fn to_box_factory<F, RET>(f: F) -> impl BoxFactory
where
    F: Fn(&'static str) -> RET,
    RET: Iterator<Item = &'static str> + 'static,
{
    move |s: &'static str| Box::new(f(s))
}

// error[E0271]: type mismatch resolving `<[closure@src/lib.rs:10:5: 10:42 f:_] as std::ops::FnOnce<(&'static str,)>>::Output == std::boxed::Box<(dyn std::iter::Iterator<Item = &'static str> + 'static)>`
//  --> src/lib.rs:5:36
//   |
// 5 | fn to_box_factory<F, RET>(f: F) -> impl BoxFactory
//   |                      ---           ^^^^^^^^^^^^^^^ expected type parameter `RET`, found trait object `dyn std::iter::Iterator`
//   |                      |
//   |                      this type parameter
//   |
//   = note: expected struct `std::boxed::Box<RET>`
//              found struct `std::boxed::Box<(dyn std::iter::Iterator<Item = &'static str> + 'static)>`
//   = help: type parameters must be constrained to match other types
//   = note: for more information, visit https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters
//   = note: required because of the requirements on the impl of `BoxFactory` for `[closure@src/lib.rs:10:5: 10:42 f:_]`
//   = note: the return type of a function must have a statically known size

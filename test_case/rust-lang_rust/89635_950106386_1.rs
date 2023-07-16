
error[E0284]: type annotations needed: cannot satisfy `<Self as Covariant<'c>>::Target == _`
 --> src/lib.rs:3:5
  |
3 |     fn unwrap<'c>(target: <Self as Covariant<'c>>::Target) -> Self where Self: Covariant<'c>;
  |        ^^^^^^ cannot satisfy `<Self as Covariant<'c>>::Target == _`

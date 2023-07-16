
error[E0599]: no method named `m` found for type parameter `T` in the current scope
 --> src/lib.rs:4:11
  |
2 |     trait Foo { fn m(self: Box<Self>); }
  |                    -       --------- the method might not be found because of this arbitrary self type
  |                    |
  |                    the method is available for `Box<T>` here
3 |     fn foo<T: Foo>(a: T) {
4 |         a.m();
  |           ^ method not found in `T`
...
8 |     trait Bar { fn m(self: std::sync::Arc<Self>); }
  |                            -------------------- the method might not be found because of this arbitrary self type
  |
help: consider wrapping the receiver expression with the appropriate type
  |
4 |         Box::new(a).m();
  |         +++++++++ +

error[E0599]: no method named `m` found for type parameter `impl Bar` in the current scope
  --> src/lib.rs:10:11
   |
2  |     trait Foo { fn m(self: Box<Self>); }
   |                            --------- the method might not be found because of this arbitrary self type
...
8  |     trait Bar { fn m(self: std::sync::Arc<Self>); }
   |                    -       -------------------- the method might not be found because of this arbitrary self type
   |                    |
   |                    the method is available for `Arc<impl Bar>` here
9  |     fn bar(b: impl Bar) {
10 |         b.m();
   |           ^ method not found in `impl Bar`
   |
help: consider wrapping the receiver expression with the appropriate type
   |
10 |         Arc::new(b).m();
   |         +++++++++ +

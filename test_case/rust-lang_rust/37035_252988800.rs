 rust
[root@li1424-173 rust]# build/x86_64-unknown-linux-gnu/stage1/bin/rustc - <<<"struct Foo<T> { inner: T } impl<T> Foo<T> { fn new<U>(u: U) -> Foo<U> { Self { inner: u } } } fn main() {}"                │··
error[E0308]: mismatched types                                                                                                                                                                           │··
 --> <anon>:1:87                                                                                                                                                                                         │··
  |                                                                                                                                                                                                      │··
1 | struct Foo<T> { inner: T } impl<T> Foo<T> { fn new<U>(u: U) -> Foo<U> { Self { inner: u } } } fn main() {}                                                                                           │··
  |                                                                                       ^ expected type parameter, found a different type parameter                                                    │··
  |                                                                                                                                                                                                      │··
  = note: expected type `T`                                                                                                                                                                              │··
  = note:    found type `U`                                                                                                                                                                              │··
                                                                                                                                                                                                         │··
error[E0308]: mismatched types                                                                                                                                                                           │··
 --> <anon>:1:73                                                                                                                                                                                         │··
  |                                                                                                                                                                                                      │··
1 | struct Foo<T> { inner: T } impl<T> Foo<T> { fn new<U>(u: U) -> Foo<U> { Self { inner: u } } } fn main() {}                                                                                           │··
  |                                                                         ^^^^^^^^^^^^^^^^^ expected type parameter, found a different type parameter                                                  │··
  |                                                                                                                                                                                                      │··
  = note: expected type `Foo<U>`                                                                                                                                                                         │··
  = note:    found type `Foo<T>`                                                                                                                                                                         │··
                                                                                                                                                                                                         │··
error: aborting due to 2 previous errors

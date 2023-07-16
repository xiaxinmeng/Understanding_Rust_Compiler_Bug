 rust
[root@li1424-173 rust]# build/x86_64-unknown-linux-gnu/stage1/bin/rustc - <<<"struct Foo<T>(T); impl<T> Foo<T> { fn new<U>(u: U) -> Foo<U> { Self(u) } } fn main() {}"                                   │··
error[E0425]: unresolved name `Self`                                                                                                                                                                     │··
 --> <anon>:1:64                                                                                                                                                                                         │··
  |                                                                                                                                                                                                      │··
1 | struct Foo<T>(T); impl<T> Foo<T> { fn new<U>(u: U) -> Foo<U> { Self(u) } } fn main() {}                                                                                                              │··
  |                                                                ^^^^ unresolved name                                                                                                                  │··
                                                                                                                                                                                                         │··
error: aborting due to previous error

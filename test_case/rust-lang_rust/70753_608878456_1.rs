
error: macro expansion ignores token `(` and any following
  --> src/lib.rs:7:32
   |
7  |         std::mem::size_of::<$t>()
   |                                ^^
...
12 | type A = Foo<n!(u8)>;
   |              ------ caused by the macro expansion here
   |
   = note: the usage of `n!` is likely invalid in type context

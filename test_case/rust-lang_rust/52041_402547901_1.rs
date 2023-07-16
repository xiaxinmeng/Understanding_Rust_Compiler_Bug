
zmd@ReflectiveCoherence:~/Code/Misc$ rustc +nightly elide_ref.rs 
warning: hidden lifetime parameters are deprecated, try `Foo<'_>`
  --> elide_ref.rs:10:13
   |
10 |     let b1: Ref<(u32, char)> = c.borrow();
   |             ^^^^^^^^^^^^^^^^
   |
note: lint level defined here
  --> elide_ref.rs:2:9
   |
2  | #![warn(elided_lifetimes_in_paths)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: hidden lifetime parameters are deprecated, try `Foo<'_>`
  --> elide_ref.rs:11:24
   |
11 |     let b2: Ref<u32> = Ref::map(b1, |t| &t.0);
   |                        ^^^^^^^^

warning: hidden lifetime parameters are deprecated, try `Foo<'_>`
  --> elide_ref.rs:11:13
   |
11 |     let b2: Ref<u32> = Ref::map(b1, |t| &t.0);
   |             ^^^^^^^^

warning: hidden lifetime parameters are deprecated, try `Foo<'_>`
  --> elide_ref.rs:12:5
   |
12 |     assert_eq!(*b2, 5);
   |     ^^^^^^^^^^^^^^^^^^^
   |
   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

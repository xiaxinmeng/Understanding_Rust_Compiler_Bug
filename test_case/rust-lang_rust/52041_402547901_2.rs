
zmd@ReflectiveCoherence:~/Code/Misc$ rustc +nightly elide_ref_fixed.rs 
warning: hidden lifetime parameters are deprecated, try `Foo<'_>`
  --> elide_ref_fixed.rs:10:17
   |
10 |     let b1: Ref<'_, (u32, char)> = c.borrow();
   |                 ^^
   |
note: lint level defined here
  --> elide_ref_fixed.rs:2:9
   |
2  | #![warn(elided_lifetimes_in_paths)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: hidden lifetime parameters are deprecated, try `Foo<'_>`
  --> elide_ref_fixed.rs:11:28
   |
11 |     let b2: Ref<'_, u32> = Ref::map(b1, |t| &t.0);
   |                            ^^^^^^^^

warning: hidden lifetime parameters are deprecated, try `Foo<'_>`
  --> elide_ref_fixed.rs:11:17
   |
11 |     let b2: Ref<'_, u32> = Ref::map(b1, |t| &t.0);
   |                 ^^

warning: hidden lifetime parameters are deprecated, try `Foo<'_>`
  --> elide_ref_fixed.rs:12:5
   |
12 |     assert_eq!(*b2, 5);
   |     ^^^^^^^^^^^^^^^^^^^
   |
   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

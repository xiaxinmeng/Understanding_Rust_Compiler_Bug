
rustc 1.16.0-beta.2 (bc15d5281 2017-02-16)
warning: function is never used: `f`, #[warn(dead_code)] on by default
 --> <anon>:3:1
  |
3 | fn f<'a>(PhantomData::<&'a u8>: PhantomData<&'a u8>) {}
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

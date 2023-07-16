
rustc 1.17.0-nightly (536a900c4 2017-02-17)
error[E0424]: expected value, found module `self`
  --> <anon>:5:40
   |
5  |             fn foo($a $($s)*) -> u32 { self.i }
   |                                        ^^^^ `self` value is only available in methods with `self` parameter
...
12 | method!(Foo, &self);
   | -------------------- in this macro invocation

error: aborting due to previous error

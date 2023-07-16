
$ (cd corevideor/ && cargo tree)
corevideor v0.1.0 (/home/joshua/test-rustdoc/rustc-84738/corevideor)
├── empty v0.1.0 (/home/joshua/test-rustdoc/rustc-84738/empty)
└── objr v0.2.0 (/home/joshua/test-rustdoc/rustc-84738/objr)
$ fd .rs | while read f; do echo $f; bat $f; done
corevideor/src/lib.rs
   1 extern crate objr;
empty/src/lib.rs
objr/src/lib.rs
   1 pub trait Trait {
   2     /// [empty]
   3     fn f() {}
   4 }
   5 impl Trait for usize {
   6     fn f() {}
   7 }

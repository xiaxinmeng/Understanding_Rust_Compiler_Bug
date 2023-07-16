
rustc: x86_64-unknown-linux-gnu/stage2/test/coretest-x86_64-unknown-linux-gnu
/home/sfackler/rust/rust/src/libcore/iter.rs:2222:21: 2222:37 error: mismatched types: expected `core::option::Option<core::cmp::Ordering>` but found `option::Option<<generic #426>>` (expected enum core::option::Option but found enum option::Option)
/home/sfackler/rust/rust/src/libcore/iter.rs:2222                     Some(cmp::Equal) => (),
                                                                      ^~~~~~~~~~~~~~~~
/home/sfackler/rust/rust/src/libcore/iter.rs:2223:38: 2223:44 error: mismatched types: expected `option::Option<core::cmp::Ordering>` but found `core::option::Option<core::cmp::Ordering>` (expected enum option::Option but found enum core::option::Option)
/home/sfackler/rust/rust/src/libcore/iter.rs:2223                     non_eq => return non_eq,
                                                                                       ^~~~~~
/home/sfackler/rust/rust/src/libcore/iter.rs:3036:13: 3038:14 error: method `partial_cmp` has an incompatible type for trait: expected enum core::option::Option but found enum option::Option
/home/sfackler/rust/rust/src/libcore/iter.rs:3036             fn partial_cmp(&self, _: &Foo) -> Option<Ordering> {
/home/sfackler/rust/rust/src/libcore/iter.rs:3037                 None
/home/sfackler/rust/rust/src/libcore/iter.rs:3038             }
/home/sfackler/rust/rust/src/libcore/option.rs:151:30: 151:40 error: mismatched types: expected `core::option::Option<core::cmp::Ordering>` but found `option::Option<core::cmp::Ordering>` (expected enum core::option::Option but found enum option::Option)
/home/sfackler/rust/rust/src/libcore/option.rs:151 #[deriving(Clone, PartialEq, PartialOrd, Eq, Ord, Show)]
                                                                                ^~~~~~~~~~
note: in expansion of #[deriving]
/home/sfackler/rust/rust/src/libcore/option.rs:151:1: 152:4 note: expansion site
/home/sfackler/rust/rust/src/libcore/option.rs:151:30: 151:40 error: if and else have incompatible types: expected `option::Option<core::cmp::Ordering>` but found `core::option::Option<core::cmp::Ordering>` (expected enum core::option::Option but found enum option::Option)
/home/sfackler/rust/rust/src/libcore/option.rs:151 #[deriving(Clone, PartialEq, PartialOrd, Eq, Ord, Show)]
                                                                                ^~~~~~~~~~
note: in expansion of #[deriving]
/home/sfackler/rust/rust/src/libcore/option.rs:151:1: 152:4 note: expansion site
/home/sfackler/rust/rust/src/libcore/option.rs:151:30: 151:40 error: method `partial_cmp` has an incompatible type for trait: expected enum core::option::Option but found enum option::Option
/home/sfackler/rust/rust/src/libcore/option.rs:151 #[deriving(Clone, PartialEq, PartialOrd, Eq, Ord, Show)]
                                                                                ^~~~~~~~~~
note: in expansion of #[deriving]
/home/sfackler/rust/rust/src/libcore/option.rs:151:1: 152:4 note: expansion site
/home/sfackler/rust/rust/src/libcore/result.rs:286:30: 286:40 error: mismatched types: expected `core::option::Option<core::cmp::Ordering>` but found `option::Option<core::cmp::Ordering>` (expected enum core::option::Option but found enum option::Option)
/home/sfackler/rust/rust/src/libcore/result.rs:286 #[deriving(Clone, PartialEq, PartialOrd, Eq, Ord, Show)]
                                                                                ^~~~~~~~~~
note: in expansion of #[deriving]
/home/sfackler/rust/rust/src/libcore/result.rs:286:1: 287:2 note: expansion site
/home/sfackler/rust/rust/src/libcore/result.rs:286:30: 286:40 error: if and else have incompatible types: expected `option::Option<core::cmp::Ordering>` but found `core::option::Option<core::cmp::Ordering>` (expected enum core::option::Option but found enum option::Option)
/home/sfackler/rust/rust/src/libcore/result.rs:286 #[deriving(Clone, PartialEq, PartialOrd, Eq, Ord, Show)]
                                                                                ^~~~~~~~~~
note: in expansion of #[deriving]
/home/sfackler/rust/rust/src/libcore/result.rs:286:1: 287:2 note: expansion site
/home/sfackler/rust/rust/src/libcore/result.rs:286:30: 286:40 error: mismatched types: expected `core::option::Option<core::cmp::Ordering>` but found `option::Option<core::cmp::Ordering>` (expected enum core::option::Option but found enum option::Option)
/home/sfackler/rust/rust/src/libcore/result.rs:286 #[deriving(Clone, PartialEq, PartialOrd, Eq, Ord, Show)]
                                                                                ^~~~~~~~~~
note: in expansion of #[deriving]
/home/sfackler/rust/rust/src/libcore/result.rs:286:1: 287:2 note: expansion site
/home/sfackler/rust/rust/src/libcore/result.rs:286:30: 286:40 error: if and else have incompatible types: expected `option::Option<core::cmp::Ordering>` but found `core::option::Option<core::cmp::Ordering>` (expected enum core::option::Option but found enum option::Option)
/home/sfackler/rust/rust/src/libcore/result.rs:286 #[deriving(Clone, PartialEq, PartialOrd, Eq, Ord, Show)]
                                                                                ^~~~~~~~~~
note: in expansion of #[deriving]
/home/sfackler/rust/rust/src/libcore/result.rs:286:1: 287:2 note: expansion site
/home/sfackler/rust/rust/src/libcore/result.rs:286:30: 286:40 error: method `partial_cmp` has an incompatible type for trait: expected enum core::option::Option but found enum option::Option
/home/sfackler/rust/rust/src/libcore/result.rs:286 #[deriving(Clone, PartialEq, PartialOrd, Eq, Ord, Show)]
                                                                                ^~~~~~~~~~
note: in expansion of #[deriving]
/home/sfackler/rust/rust/src/libcore/result.rs:286:1: 287:2 note: expansion site
error: aborting due to 11 previous errors
/home/sfackler/rust/rust/mk/tests.mk:384: recipe for target 'x86_64-unknown-linux-gnu/stage2/test/coretest-x86_64-unknown-linux-gnu' failed
make: *** [x86_64-unknown-linux-gnu/stage2/test/coretest-x86_64-unknown-linux-gnu] Error 101

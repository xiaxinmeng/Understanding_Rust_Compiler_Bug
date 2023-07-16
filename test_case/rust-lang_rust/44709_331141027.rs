
[00:48:08] error: `..=` syntax in patterns is experimental (see issue #28237)
[00:48:08]   --> /checkout/src/test/run-pass/inc-range-pat.rs:15:24
[00:48:08]    |
[00:48:08] 15 |     assert!(match 42 { 0 ..= 100 => true, _ => false });
[00:48:08]    |                        ^^^^^^^^^
[00:48:08]    |
[00:48:08]    = help: add #![feature(dotdoteq_in_patterns)] to the crate attributes to enable

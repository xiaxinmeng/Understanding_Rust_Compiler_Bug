
[00:03:47] error: this file contains an un-closed delimiter
[00:03:47]     --> libcore/num/mod.rs:3756:4
[00:03:47]      |
[00:03:47] 3756 | ];
[00:03:47]      |    ^
[00:03:47]      |
[00:03:47] help: did you mean to close this delimiter?
[00:03:47]     --> libcore/num/mod.rs:1390:24
[00:03:47]      |
[00:03:47] 1390 | macro_rules! uint_impl {
[00:03:47]      |                        ^
[00:03:47] 
[00:03:47] error: no rules expected the token `#`
[00:03:47]     --> libcore/num/mod.rs:2481:1
[00:03:47]      |
[00:03:47] 2481 | #[lang = "u8"]
[00:03:47]      | ^
[00:03:47] 
[00:03:47] error: Could not compile `core`.

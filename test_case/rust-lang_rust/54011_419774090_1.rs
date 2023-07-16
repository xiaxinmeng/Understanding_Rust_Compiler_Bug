
[00:49:11] error: `serde` import is ambiguous
[00:49:11]    --> tools\clippy\clippy_lints\src\utils\conf.rs:81:25
[00:49:11]     |
[00:49:11] 80  |                       use serde;
[00:49:11]     |                           ----- may refer to `self::serde` in the future
[00:49:11] 81  |                       use serde::Deserialize;
[00:49:11]     |                           ^^^^^ can refer to external crate `::serde`
[00:49:11] ...
[00:49:11] 108 | / define_Conf! {
[00:49:11] 109 | |     /// Lint: BLACKLISTED_NAME. The list of blacklisted names to lint about
[00:49:11] 110 | |     (blacklisted_names, "blacklisted_names", ["foo", "bar", "baz", "quux"] => Vec<String>),
[00:49:11] 111 | |     /// Lint: CYCLOMATIC_COMPLEXITY. The maximum cyclomatic complexity a function can have
[00:49:11] ...   |
[00:49:11] 147 | |     (trivial_copy_size_limit, "trivial_copy_size_limit", None => Option<u64>),
[00:49:11] 148 | | }
[00:49:11]     | |_- in this macro invocation
[00:49:11]     |
[00:49:11]     = help: write `::serde` or `self::serde` explicitly instead
[00:49:11]     = note: in the future, `#![feature(uniform_paths)]` may become the default
[00:49:11] 
[00:49:11] error: `std` import is ambiguous
[00:49:11]   --> tools\clippy\clippy_lints\src\utils\sugg.rs:8:5
[00:49:11]    |
[00:49:11] 8  | use std::borrow::Cow;
[00:49:11]    |     ^^^ can refer to external crate `::std`
[00:49:11] 9  | use std::fmt::Display;
[00:49:11] 10 | use std;
[00:49:11]    |     --- may refer to `self::std` in the future
[00:49:11]    |
[00:49:11]    = help: write `::std` or `self::std` explicitly instead
[00:49:11]    = note: in the future, `#![feature(uniform_paths)]` may become the default
...

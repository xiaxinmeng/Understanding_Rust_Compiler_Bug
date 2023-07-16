
~/dev/rust_inline_bug main> cargo +nightly run
   Compiling advent_of_code v0.1.0 (/home/jonas/dev/rust_inline_bug)
error[E0432]: unresolved import `regex`
 --> src/y2015/day_07.rs:1:5
  |
1 | use regex::Regex;
  |     ^^^^^ use of undeclared crate or module `regex`

error[E0432]: unresolved import `substring`
 --> src/y2015/day_07.rs:2:5
  |
2 | use substring::Substring;
  |     ^^^^^^^^^ use of undeclared crate or module `substring`

error[E0432]: unresolved import `regex`
 --> src/y2015/day_08.rs:1:5
  |
1 | use regex::Regex;
  |     ^^^^^ use of undeclared crate or module `regex`

error[E0432]: unresolved import `serde_json`
 --> src/y2015/day_12.rs:2:5
  |
2 | use serde_json::Value;
  |     ^^^^^^^^^^ use of undeclared crate or module `serde_json`

error[E0433]: failed to resolve: use of undeclared crate or module `md5`
  --> src/y2015/day_04.rs:12:20
   |
12 |         let hash = md5::compute(format!("{}{}", INPUT, i));
   |                    ^^^ use of undeclared crate or module `md5`

error[E0433]: failed to resolve: use of undeclared crate or module `md5`
  --> src/y2016/day_05.rs:10:20
   |
10 |         let hash = md5::compute(format!("{}{}", INPUT, i));
   |                    ^^^ use of undeclared crate or module `md5`

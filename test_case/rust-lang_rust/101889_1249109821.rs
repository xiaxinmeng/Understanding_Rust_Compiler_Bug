plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
........................................................................................ 1144/3881
........................................................................................ 1232/3881
........................................................................................ 1320/3881
........................................................................................ 1408/3881
..................................................................................F.F... 1496/3881
.....................................................................................FF. 1584/3881
F..F.................................................................................... 1760/3881
F..F.................................................................................... 1760/3881
........F.F............................................................................. 1848/3881
..............FF........................................................................ 1936/3881
.....................................F...F.............................................. 2024/3881
........................................................................................ 2200/3881
........................................................................................ 2288/3881
........................................................................................ 2376/3881
........................................................................................ 2464/3881
---
---- src/num/mod.rs - num::u128::borrowing_sub (line 884) stdout ----
error: mismatched closing delimiter: `}`
  --> src/num/mod.rs:901:11
   |
10 | fn main() { #[allow(non_snake_case)] fn _doctest_main_src_num_mod_rs_884_2() {
...
...
20 | assert_eq!((diff1, diff0), (3, u128::MAX);
   |           ^ unclosed delimiter
21 | } _doctest_main_src_num_mod_rs_884_2() }
   | ^ mismatched closing delimiter

error: no rules expected the token `;`
   |
   |
20 | assert_eq!((diff1, diff0), (3, u128::MAX);
   |                                          |
   |                                          no rules expected this token in macro call
   |                                          help: missing comma here


error[E0599]: no method named `carrying_sub` found for type `{integer}` in the current scope
   |
   |
15 | let (diff0, borrow1) = a0.carrying_sub(b0, borrow0);
   |                           ^^^^^^^^^^^^ method not found in `{integer}`

error[E0599]: no method named `carrying_sub` found for type `{integer}` in the current scope
   |
   |
17 | let (diff1, borrow2) = a1.carrying_sub(b1, borrow1);
   |                           ^^^^^^^^^^^^ method not found in `{integer}`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0599`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::u128::carrying_add (line 891) stdout ----
error[E0689]: can't call method `carrying_add` on ambiguous numeric type `{integer}`
   |
   |
11 | let (a1, a0) = (3, u128::MAX);
   |      -- you must specify a type for this binding, like `i32`
...
17 | let (sum1, carry2) = a1.carrying_add(b1, carry1);

error: aborting due to previous error

For more information about this error, try `rustc --explain E0689`.
For more information about this error, try `rustc --explain E0689`.
Couldn't compile the test.
---- src/num/mod.rs - num::u16::borrowing_sub (line 838) stdout ----
error: mismatched closing delimiter: `}`
  --> src/num/mod.rs:855:11
   |
10 | fn main() { #[allow(non_snake_case)] fn _doctest_main_src_num_mod_rs_838_2() {
...
...
20 | assert_eq!((diff1, diff0), (3, u16::MAX);
   |           ^ unclosed delimiter
21 | } _doctest_main_src_num_mod_rs_838_2() }
   | ^ mismatched closing delimiter

error: no rules expected the token `;`
   |
   |
20 | assert_eq!((diff1, diff0), (3, u16::MAX);
   |                                         |
   |                                         no rules expected this token in macro call
   |                                         help: missing comma here


error[E0599]: no method named `carrying_sub` found for type `{integer}` in the current scope
   |
   |
15 | let (diff0, borrow1) = a0.carrying_sub(b0, borrow0);
   |                           ^^^^^^^^^^^^ method not found in `{integer}`

error[E0599]: no method named `carrying_sub` found for type `{integer}` in the current scope
   |
   |
17 | let (diff1, borrow2) = a1.carrying_sub(b1, borrow1);
   |                           ^^^^^^^^^^^^ method not found in `{integer}`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0599`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::u16::carrying_add (line 845) stdout ----
error[E0689]: can't call method `carrying_add` on ambiguous numeric type `{integer}`
   |
   |
11 | let (a1, a0) = (3, u16::MAX);
   |      -- you must specify a type for this binding, like `i32`
...
17 | let (sum1, carry2) = a1.carrying_add(b1, carry1);

error: aborting due to previous error

For more information about this error, try `rustc --explain E0689`.
For more information about this error, try `rustc --explain E0689`.
Couldn't compile the test.
---- src/num/mod.rs - num::u32::borrowing_sub (line 869) stdout ----
error: mismatched closing delimiter: `}`
  --> src/num/mod.rs:886:11
   |
10 | fn main() { #[allow(non_snake_case)] fn _doctest_main_src_num_mod_rs_869_2() {
...
...
20 | assert_eq!((diff1, diff0), (3, u32::MAX);
   |           ^ unclosed delimiter
21 | } _doctest_main_src_num_mod_rs_869_2() }
   | ^ mismatched closing delimiter

error: no rules expected the token `;`
   |
   |
20 | assert_eq!((diff1, diff0), (3, u32::MAX);
   |                                         |
   |                                         no rules expected this token in macro call
   |                                         help: missing comma here


error[E0599]: no method named `carrying_sub` found for type `{integer}` in the current scope
   |
   |
15 | let (diff0, borrow1) = a0.carrying_sub(b0, borrow0);
   |                           ^^^^^^^^^^^^ method not found in `{integer}`

error[E0599]: no method named `carrying_sub` found for type `{integer}` in the current scope
   |
   |
17 | let (diff1, borrow2) = a1.carrying_sub(b1, borrow1);
   |                           ^^^^^^^^^^^^ method not found in `{integer}`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0599`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::u32::carrying_add (line 876) stdout ----
error[E0689]: can't call method `carrying_add` on ambiguous numeric type `{integer}`
   |
   |
11 | let (a1, a0) = (3, u32::MAX);
   |      -- you must specify a type for this binding, like `i32`
...
17 | let (sum1, carry2) = a1.carrying_add(b1, carry1);

error: aborting due to previous error

For more information about this error, try `rustc --explain E0689`.
For more information about this error, try `rustc --explain E0689`.
Couldn't compile the test.
---- src/num/mod.rs - num::u64::borrowing_sub (line 875) stdout ----
error: mismatched closing delimiter: `}`
  --> src/num/mod.rs:892:11
   |
10 | fn main() { #[allow(non_snake_case)] fn _doctest_main_src_num_mod_rs_875_3() {
...
...
20 | assert_eq!((diff1, diff0), (3, u64::MAX);
   |           ^ unclosed delimiter
21 | } _doctest_main_src_num_mod_rs_875_3() }
   | ^ mismatched closing delimiter

error: no rules expected the token `;`
   |
   |
20 | assert_eq!((diff1, diff0), (3, u64::MAX);
   |                                         |
   |                                         no rules expected this token in macro call
error: test failed, to rerun pass '--doc'
   |                                         help: missing comma here
   |                                         help: missing comma here

error[E0599]: no method named `carrying_sub` found for type `{integer}` in the current scope
   |
   |
15 | let (diff0, borrow1) = a0.carrying_sub(b0, borrow0);
   |                           ^^^^^^^^^^^^ method not found in `{integer}`

error[E0599]: no method named `carrying_sub` found for type `{integer}` in the current scope
   |
   |
17 | let (diff1, borrow2) = a1.carrying_sub(b1, borrow1);
   |                           ^^^^^^^^^^^^ method not found in `{integer}`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0599`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::u64::carrying_add (line 882) stdout ----
error[E0689]: can't call method `carrying_add` on ambiguous numeric type `{integer}`
   |
   |
11 | let (a1, a0) = (3, u64::MAX);
   |      -- you must specify a type for this binding, like `i32`
...
17 | let (sum1, carry2) = a1.carrying_add(b1, carry1);

error: aborting due to previous error

For more information about this error, try `rustc --explain E0689`.
For more information about this error, try `rustc --explain E0689`.
Couldn't compile the test.
---- src/num/mod.rs - num::u8::borrowing_sub (line 285) stdout ----
error: mismatched closing delimiter: `}`
  --> src/num/mod.rs:302:11
   |
10 | fn main() { #[allow(non_snake_case)] fn _doctest_main_src_num_mod_rs_285_2() {
...
...
20 | assert_eq!((diff1, diff0), (3, u8::MAX);
   |           ^ unclosed delimiter
21 | } _doctest_main_src_num_mod_rs_285_2() }
   | ^ mismatched closing delimiter

error: no rules expected the token `;`
   |
   |
20 | assert_eq!((diff1, diff0), (3, u8::MAX);
   |                                        |
   |                                        no rules expected this token in macro call
   |                                        help: missing comma here


error[E0599]: no method named `carrying_sub` found for type `{integer}` in the current scope
   |
   |
15 | let (diff0, borrow1) = a0.carrying_sub(b0, borrow0);
   |                           ^^^^^^^^^^^^ method not found in `{integer}`

error[E0599]: no method named `carrying_sub` found for type `{integer}` in the current scope
   |
   |
17 | let (diff1, borrow2) = a1.carrying_sub(b1, borrow1);
   |                           ^^^^^^^^^^^^ method not found in `{integer}`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0599`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::u8::carrying_add (line 292) stdout ----
error[E0689]: can't call method `carrying_add` on ambiguous numeric type `{integer}`
   |
   |
11 | let (a1, a0) = (3, u8::MAX);
   |      -- you must specify a type for this binding, like `i32`
...
17 | let (sum1, carry2) = a1.carrying_add(b1, carry1);

error: aborting due to previous error

For more information about this error, try `rustc --explain E0689`.
For more information about this error, try `rustc --explain E0689`.
Couldn't compile the test.
---- src/num/mod.rs - num::usize::borrowing_sub (line 913) stdout ----
error: mismatched closing delimiter: `}`
  --> src/num/mod.rs:930:11
   |
10 | fn main() { #[allow(non_snake_case)] fn _doctest_main_src_num_mod_rs_913_2() {
...
...
20 | assert_eq!((diff1, diff0), (3, usize::MAX);
   |           ^ unclosed delimiter
21 | } _doctest_main_src_num_mod_rs_913_2() }
   | ^ mismatched closing delimiter

error: no rules expected the token `;`
   |
   |
20 | assert_eq!((diff1, diff0), (3, usize::MAX);
   |                                           |
   |                                           no rules expected this token in macro call
   |                                           help: missing comma here


error[E0599]: no method named `carrying_sub` found for type `{integer}` in the current scope
   |
   |
15 | let (diff0, borrow1) = a0.carrying_sub(b0, borrow0);
   |                           ^^^^^^^^^^^^ method not found in `{integer}`

error[E0599]: no method named `carrying_sub` found for type `{integer}` in the current scope
   |
   |
17 | let (diff1, borrow2) = a1.carrying_sub(b1, borrow1);
   |                           ^^^^^^^^^^^^ method not found in `{integer}`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0599`.
Couldn't compile the test.
Couldn't compile the test.
---- src/num/mod.rs - num::usize::carrying_add (line 920) stdout ----
error[E0689]: can't call method `carrying_add` on ambiguous numeric type `{integer}`
   |
   |
11 | let (a1, a0) = (3, usize::MAX);
   |      -- you must specify a type for this binding, like `i32`
...
17 | let (sum1, carry2) = a1.carrying_add(b1, carry1);

error: aborting due to previous error

For more information about this error, try `rustc --explain E0689`.

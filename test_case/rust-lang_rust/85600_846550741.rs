plain
   Compiling libc v0.2.93
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.43
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0658]: arbitrary expressions in key-value attributes are unstable
  |
  |
1 | #[doc = include_str!("panic.md")]
  |
  = note: see issue #78835 <https://github.com/rust-lang/rust/issues/78835> for more information
  = help: add `#![feature(extended_key_value_attributes)]` to the crate attributes to enable


error[E0658]: arbitrary expressions in key-value attributes are unstable
   |
   |
11 | #[doc = include_str!("core_arch_docs.md")]
   |
   = note: see issue #78835 <https://github.com/rust-lang/rust/issues/78835> for more information
   = help: add `#![feature(extended_key_value_attributes)]` to the crate attributes to enable


error[E0658]: arbitrary expressions in key-value attributes are unstable
   |
3  |  / macro_rules! int_module {
3  |  / macro_rules! int_module {
4  |  |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
5  |  |     ($T:ident, #[$attr:meta]) => (
6  |  |         #[doc = concat!(
   |  |_________________^
7  | ||             "The smallest value that can be represented by this integer type. Use ",
8  | ||             "[`", stringify!($T), "::MIN", "`] instead."
9  | ||         )]
...   |
43 |  |     )
44 |  | }
   |  |_- in this expansion of `int_module!`
   |  |_- in this expansion of `int_module!`
   | 
  ::: library/core/src/num/shells/i128.rs:13:1
   |
13 |    int_module! { i128, #[stable(feature = "i128", since="1.26.0")] }
   |
   = note: see issue #78835 <https://github.com/rust-lang/rust/issues/78835> for more information
   = help: add `#![feature(extended_key_value_attributes)]` to the crate attributes to enable


error[E0658]: arbitrary expressions in key-value attributes are unstable
   |
3  | / macro_rules! int_module {
3  | / macro_rules! int_module {
4  | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
5  | |     ($T:ident, #[$attr:meta]) => (
6  | |         #[doc = concat!(
...  |
15 | |         #[doc = concat!("let min = std::", stringify!($T), "::MIN;")]
...  |
43 | |     )
44 | | }
   | |_- in this expansion of `int_module!`
   | |_- in this expansion of `int_module!`
   | 
  ::: library/core/src/num/shells/i128.rs:13:1
   |
13 |   int_module! { i128, #[stable(feature = "i128", since="1.26.0")] }
   |
   = note: see issue #78835 <https://github.com/rust-lang/rust/issues/78835> for more information
   = help: add `#![feature(extended_key_value_attributes)]` to the crate attributes to enable


error[E0658]: arbitrary expressions in key-value attributes are unstable
   |
3  | / macro_rules! int_module {
3  | / macro_rules! int_module {
4  | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
5  | |     ($T:ident, #[$attr:meta]) => (
6  | |         #[doc = concat!(
...  |
18 | |         #[doc = concat!("let min = ", stringify!($T), "::MIN;")]
...  |
43 | |     )
44 | | }
   | |_- in this expansion of `int_module!`
   | |_- in this expansion of `int_module!`
   | 
  ::: library/core/src/num/shells/i128.rs:13:1
   |
13 |   int_module! { i128, #[stable(feature = "i128", since="1.26.0")] }
   |
   = note: see issue #78835 <https://github.com/rust-lang/rust/issues/78835> for more information
   = help: add `#![feature(extended_key_value_attributes)]` to the crate attributes to enable


error[E0658]: arbitrary expressions in key-value attributes are unstable
   |
3  |  / macro_rules! int_module {
3  |  / macro_rules! int_module {
4  |  |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
5  |  |     ($T:ident, #[$attr:meta]) => (
6  |  |         #[doc = concat!(
...   |
25 |  |         #[doc = concat!(
   |  |_________________^
26 | ||             "The largest value that can be represented by this integer type. Use ",
27 | ||             "[`", stringify!($T), "::MAX", "`] instead."
28 | ||         )]
...   |
43 |  |     )
44 |  | }
   |  |_- in this expansion of `int_module!`
   |  |_- in this expansion of `int_module!`
   | 
  ::: library/core/src/num/shells/i128.rs:13:1
   |
13 |    int_module! { i128, #[stable(feature = "i128", since="1.26.0")] }
   |
   = note: see issue #78835 <https://github.com/rust-lang/rust/issues/78835> for more information
   = help: add `#![feature(extended_key_value_attributes)]` to the crate attributes to enable


error[E0658]: arbitrary expressions in key-value attributes are unstable
   |
3  | / macro_rules! int_module {
3  | / macro_rules! int_module {
4  | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
5  | |     ($T:ident, #[$attr:meta]) => (
6  | |         #[doc = concat!(
...  |
34 | |         #[doc = concat!("let max = std::", stringify!($T), "::MAX;")]
...  |
43 | |     )
44 | | }
   | |_- in this expansion of `int_module!`
   | |_- in this expansion of `int_module!`
   | 
  ::: library/core/src/num/shells/i128.rs:13:1
   |
13 |   int_module! { i128, #[stable(feature = "i128", since="1.26.0")] }
   |
   = note: see issue #78835 <https://github.com/rust-lang/rust/issues/78835> for more information
   = help: add `#![feature(extended_key_value_attributes)]` to the crate attributes to enable


error[E0658]: arbitrary expressions in key-value attributes are unstable
   |
3  | / macro_rules! int_module {
3  | / macro_rules! int_module {
4  | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
5  | |     ($T:ident, #[$attr:meta]) => (
6  | |         #[doc = concat!(
...  |
37 | |         #[doc = concat!("let max = ", stringify!($T), "::MAX;")]
...  |
43 | |     )
44 | | }
   | |_- in this expansion of `int_module!`
   | |_- in this expansion of `int_module!`
   | 
  ::: library/core/src/num/shells/i128.rs:13:1
   |
13 |   int_module! { i128, #[stable(feature = "i128", since="1.26.0")] }
   |
   = note: see issue #78835 <https://github.com/rust-lang/rust/issues/78835> for more information
   = help: add `#![feature(extended_key_value_attributes)]` to the crate attributes to enable


error[E0658]: arbitrary expressions in key-value attributes are unstable
   |
3  |  / macro_rules! int_module {
3  |  / macro_rules! int_module {
4  |  |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   |  |                    --------------------------------------------------------------- in this macro invocation (#2)
5  |  |     ($T:ident, #[$attr:meta]) => (
6  |  |         #[doc = concat!(
   |  |_________________^
7  | ||             "The smallest value that can be represented by this integer type. Use ",
8  | ||             "[`", stringify!($T), "::MIN", "`] instead."
9  | ||         )]
...   |
43 |  |     )
44 |  | }
   |  | -
   |  | -
   |  | |
   |  |_in this expansion of `int_module!` (#1)
   |    in this expansion of `int_module!` (#2)
  ::: library/core/src/num/shells/i16.rs:13:1
   |
   |
13 |    int_module! { i16 }
   |    ------------------- in this macro invocation (#1)
   = note: see issue #78835 <https://github.com/rust-lang/rust/issues/78835> for more information
   = help: add `#![feature(extended_key_value_attributes)]` to the crate attributes to enable


error[E0658]: arbitrary expressions in key-value attributes are unstable
   |
3  | / macro_rules! int_module {
3  | / macro_rules! int_module {
4  | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   | |                    --------------------------------------------------------------- in this macro invocation (#2)
5  | |     ($T:ident, #[$attr:meta]) => (
6  | |         #[doc = concat!(
...  |
15 | |         #[doc = concat!("let min = std::", stringify!($T), "::MIN;")]
...  |
43 | |     )
44 | | }
   | | -
   | | -
   | | |
   | |_in this expansion of `int_module!` (#1)
   |   in this expansion of `int_module!` (#2)
  ::: library/core/src/num/shells/i16.rs:13:1
   |
   |
13 |   int_module! { i16 }
   |   ------------------- in this macro invocation (#1)
   = note: see issue #78835 <https://github.com/rust-lang/rust/issues/78835> for more information
   = help: add `#![feature(extended_key_value_attributes)]` to the crate attributes to enable


error[E0658]: arbitrary expressions in key-value attributes are unstable
   |
3  | / macro_rules! int_module {
3  | / macro_rules! int_module {
4  | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   | |                    --------------------------------------------------------------- in this macro invocation (#2)
5  | |     ($T:ident, #[$attr:meta]) => (
6  | |         #[doc = concat!(
...  |
18 | |         #[doc = concat!("let min = ", stringify!($T), "::MIN;")]
...  |
43 | |     )
44 | | }
   | | -
   | | -
   | | |
   | |_in this expansion of `int_module!` (#1)
   |   in this expansion of `int_module!` (#2)
  ::: library/core/src/num/shells/i16.rs:13:1
   |
   |
13 |   int_module! { i16 }
   |   ------------------- in this macro invocation (#1)
   = note: see issue #78835 <https://github.com/rust-lang/rust/issues/78835> for more information
   = help: add `#![feature(extended_key_value_attributes)]` to the crate attributes to enable


error[E0658]: arbitrary expressions in key-value attributes are unstable
   |
3  |  / macro_rules! int_module {
3  |  / macro_rules! int_module {
4  |  |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   |  |                    --------------------------------------------------------------- in this macro invocation (#2)
5  |  |     ($T:ident, #[$attr:meta]) => (
6  |  |         #[doc = concat!(
...   |
25 |  |         #[doc = concat!(
   |  |_________________^
26 | ||             "The largest value that can be represented by this integer type. Use ",
27 | ||             "[`", stringify!($T), "::MAX", "`] instead."
28 | ||         )]
...   |
43 |  |     )
44 |  | }
   |  | -
   |  | -
   |  | |
   |  |_in this expansion of `int_module!` (#1)
   |    in this expansion of `int_module!` (#2)
  ::: library/core/src/num/shells/i16.rs:13:1
   |
   |
13 |    int_module! { i16 }
   |    ------------------- in this macro invocation (#1)
   = note: see issue #78835 <https://github.com/rust-lang/rust/issues/78835> for more information
   = help: add `#![feature(extended_key_value_attributes)]` to the crate attributes to enable


error[E0658]: arbitrary expressions in key-value attributes are unstable
   |
3  | / macro_rules! int_module {
3  | / macro_rules! int_module {
4  | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   | |                    --------------------------------------------------------------- in this macro invocation (#2)
5  | |     ($T:ident, #[$attr:meta]) => (
6  | |         #[doc = concat!(
...  |
34 | |         #[doc = concat!("let max = std::", stringify!($T), "::MAX;")]
...  |
43 | |     )
44 | | }
   | | -
   | | -
   | | |
   | |_in this expansion of `int_module!` (#1)
   |   in this expansion of `int_module!` (#2)
  ::: library/core/src/num/shells/i16.rs:13:1
   |
   |
13 |   int_module! { i16 }
   |   ------------------- in this macro invocation (#1)
   = note: see issue #78835 <https://github.com/rust-lang/rust/issues/78835> for more information
   = help: add `#![feature(extended_key_value_attributes)]` to the crate attributes to enable


error[E0658]: arbitrary expressions in key-value attributes are unstable
   |
3  | / macro_rules! int_module {
3  | / macro_rules! int_module {
4  | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   | |                    --------------------------------------------------------------- in this macro invocation (#2)
5  | |     ($T:ident, #[$attr:meta]) => (
6  | |         #[doc = concat!(
...  |
37 | |         #[doc = concat!("let max = ", stringify!($T), "::MAX;")]
...  |
43 | |     )
44 | | }
   | | -
   | | -
   | | |
   | |_in this expansion of `int_module!` (#1)
   |   in this expansion of `int_module!` (#2)
  ::: library/core/src/num/shells/i16.rs:13:1
   |
   |
13 |   int_module! { i16 }
   |   ------------------- in this macro invocation (#1)
   = note: see issue #78835 <https://github.com/rust-lang/rust/issues/78835> for more information
   = help: add `#![feature(extended_key_value_attributes)]` to the crate attributes to enable


error[E0658]: arbitrary expressions in key-value attributes are unstable
   |
3  |  / macro_rules! int_module {
3  |  / macro_rules! int_module {
4  |  |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   |  |                    --------------------------------------------------------------- in this macro invocation (#2)
5  |  |     ($T:ident, #[$attr:meta]) => (
6  |  |         #[doc = concat!(
   |  |_________________^
7  | ||             "The smallest value that can be represented by this integer type. Use ",
8  | ||             "[`", stringify!($T), "::MIN", "`] instead."
9  | ||         )]
...   |
43 |  |     )
44 |  | }
   |  | -
   |  | -
   |  | |
   |  |_in this expansion of `int_module!` (#1)
   |    in this expansion of `int_module!` (#2)
  ::: library/core/src/num/shells/i32.rs:13:1
   |
   |
13 |    int_module! { i32 }
   |    ------------------- in this macro invocation (#1)
   = note: see issue #78835 <https://github.com/rust-lang/rust/issues/78835> for more information
   = help: add `#![feature(extended_key_value_attributes)]` to the crate attributes to enable


error[E0658]: arbitrary expressions in key-value attributes are unstable
   |
3  | / macro_rules! int_module {
3  | / macro_rules! int_module {
4  | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   | |                    --------------------------------------------------------------- in this macro invocation (#2)
5  | |     ($T:ident, #[$attr:meta]) => (
6  | |         #[doc = concat!(
...  |
15 | |         #[doc = concat!("let min = std::", stringify!($T), "::MIN;")]
...  |
43 | |     )
44 | | }
   | | -
   | | -
   | | |
   | |_in this expansion of `int_module!` (#1)
   |   in this expansion of `int_module!` (#2)
  ::: library/core/src/num/shells/i32.rs:13:1
   |
   |
13 |   int_module! { i32 }
   |   ------------------- in this macro invocation (#1)
   = note: see issue #78835 <https://github.com/rust-lang/rust/issues/78835> for more information
   = help: add `#![feature(extended_key_value_attributes)]` to the crate attributes to enable


error[E0658]: arbitrary expressions in key-value attributes are unstable
   |
3  | / macro_rules! int_module {
3  | / macro_rules! int_module {
4  | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   | |                    --------------------------------------------------------------- in this macro invocation (#2)
5  | |     ($T:ident, #[$attr:meta]) => (
6  | |         #[doc = concat!(
...  |
18 | |         #[doc = concat!("let min = ", stringify!($T), "::MIN;")]
...  |
43 | |     )
44 | | }
   | | -
   | | -
   | | |
   | |_in this expansion of `int_module!` (#1)
   |   in this expansion of `int_module!` (#2)
  ::: library/core/src/num/shells/i32.rs:13:1
   |
   |
13 |   int_module! { i32 }
   |   ------------------- in this macro invocation (#1)
   = note: see issue #78835 <https://github.com/rust-lang/rust/issues/78835> for more information
   = help: add `#![feature(extended_key_value_attributes)]` to the crate attributes to enable


error[E0658]: arbitrary expressions in key-value attributes are unstable
   |
3  |  / macro_rules! int_module {
3  |  / macro_rules! int_module {
4  |  |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   |  |                    --------------------------------------------------------------- in this macro invocation (#2)
5  |  |     ($T:ident, #[$attr:meta]) => (
6  |  |         #[doc = concat!(
...   |
25 |  |         #[doc = concat!(
   |  |_________________^
26 | ||             "The largest value that can be represented by this integer type. Use ",
27 | ||             "[`", stringify!($T), "::MAX", "`] instead."
28 | ||         )]
...   |
43 |  |     )
44 |  | }
   |  | -
   |  | -
   |  | |
   |  |_in this expansion of `int_module!` (#1)
   |    in this expansion of `int_module!` (#2)
  ::: library/core/src/num/shells/i32.rs:13:1
   |
   |
13 |    int_module! { i32 }
   |    ------------------- in this macro invocation (#1)
   = note: see issue #78835 <https://github.com/rust-lang/rust/issues/78835> for more information
   = help: add `#![feature(extended_key_value_attributes)]` to the crate attributes to enable


error[E0658]: arbitrary expressions in key-value attributes are unstable
   |
3  | / macro_rules! int_module {
3  | / macro_rules! int_module {
4  | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   | |                    --------------------------------------------------------------- in this macro invocation (#2)
5  | |     ($T:ident, #[$attr:meta]) => (
6  | |         #[doc = concat!(
...  |
34 | |         #[doc = concat!("let max = std::", stringify!($T), "::MAX;")]
...  |
43 | |     )
44 | | }
   | | -
   | | -
   | | |
   | |_in this expansion of `int_module!` (#1)
   |   in this expansion of `int_module!` (#2)
  ::: library/core/src/num/shells/i32.rs:13:1
   |
   |
13 |   int_module! { i32 }
   |   ------------------- in this macro invocation (#1)
   = note: see issue #78835 <https://github.com/rust-lang/rust/issues/78835> for more information
   = help: add `#![feature(extended_key_value_attributes)]` to the crate attributes to enable


error[E0658]: arbitrary expressions in key-value attributes are unstable
   |
3  | / macro_rules! int_module {
3  | / macro_rules! int_module {
4  | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   | |                    --------------------------------------------------------------- in this macro invocation (#2)
5  | |     ($T:ident, #[$attr:meta]) => (
6  | |         #[doc = concat!(
...  |
37 | |         #[doc = concat!("let max = ", stringify!($T), "::MAX;")]
...  |
43 | |     )
44 | | }
   | | -
   | | -
   | | |
   | |_in this expansion of `int_module!` (#1)
   |   in this expansion of `int_module!` (#2)
  ::: library/core/src/num/shells/i32.rs:13:1
   |
   |
13 |   int_module! { i32 }
   |   ------------------- in this macro invocation (#1)
   = note: see issue #78835 <https://github.com/rust-lang/rust/issues/78835> for more information
   = help: add `#![feature(extended_key_value_attributes)]` to the crate attributes to enable

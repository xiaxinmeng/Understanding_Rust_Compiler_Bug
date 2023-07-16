plain
[command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :
[command]/usr/bin/git config --local http.https://github.com/.extraheader AUTHORIZATION: basic ***
##[endgroup]
##[group]Fetching the repository
[command]/usr/bin/git -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=2 origin +8390e2cf97dedbc142df480388bd403520839b89:refs/remotes/pull/79263/merge
---
   Compiling libc v0.2.79
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.36
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: unexpected token: `(/*ERROR*/)`
   |
3  | / macro_rules! doc_comment {
3  | / macro_rules! doc_comment {
4  | |     ($x:expr, $($tt:tt)*) => {
5  | |         #[doc = $x]
   | |                 ^^
6  | |         $($tt)*
8  | | }
8  | | }
   | |_- in this expansion of `doc_comment!` (#2)
10 | / macro_rules! int_module {
10 | / macro_rules! int_module {
11 | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
12 | |     ($T:ident, #[$attr:meta]) => (
13 |           doc_comment! {
   |  _________-
14 |               concat!("The smallest value that can be represented by this integer type.
15 |   Use [`", stringify!($T), "::MIN", "`](../../std/primitive.", stringify!($T), ".html#associatedconstant.MIN) instead.
...
...
28 |               pub const MIN: $T = $T::MIN;
   | |_________- in this macro invocation (#2)
...
48 | |     )
49 | | }
49 | | }
   | |_- in this expansion of `int_module!` (#1)
  ::: library/core/src/num/shells/i128.rs:10:1
   |
   |
10 |   int_module! { i128, #[stable(feature = "i128", since="1.26.0")] }
   |   ----------------------------------------------------------------- in this macro invocation (#1)

error: unexpected token: `(/*ERROR*/)`
   |
3  | / macro_rules! doc_comment {
3  | / macro_rules! doc_comment {
4  | |     ($x:expr, $($tt:tt)*) => {
5  | |         #[doc = $x]
   | |                 ^^
6  | |         $($tt)*
8  | | }
8  | | }
   | |_- in this expansion of `doc_comment!` (#2)
10 | / macro_rules! int_module {
10 | / macro_rules! int_module {
11 | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
12 | |     ($T:ident, #[$attr:meta]) => (
13 | |         doc_comment! {
31 | /         doc_comment! {
31 | /         doc_comment! {
32 |               concat!("The largest value that can be represented by this integer type.
33 |   Use [`", stringify!($T), "::MAX", "`](../../std/primitive.", stringify!($T), ".html#associatedconstant.MAX) instead.
...
...
46 |               pub const MAX: $T = $T::MAX;
   | |_________- in this macro invocation (#2)
48 | |     )
49 | | }
49 | | }
   | |_- in this expansion of `int_module!` (#1)
  ::: library/core/src/num/shells/i128.rs:10:1
   |
   |
10 |   int_module! { i128, #[stable(feature = "i128", since="1.26.0")] }
   |   ----------------------------------------------------------------- in this macro invocation (#1)

error: unexpected token: `(/*ERROR*/)`
   |
3  | / macro_rules! doc_comment {
3  | / macro_rules! doc_comment {
4  | |     ($x:expr, $($tt:tt)*) => {
5  | |         #[doc = $x]
   | |                 ^^
6  | |         $($tt)*
8  | | }
8  | | }
   | |_- in this expansion of `doc_comment!` (#3)
10 |   macro_rules! int_module {
   |  _-
   | |_|
   | |
   | |
11 | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   | |                    --------------------------------------------------------------- in this macro invocation (#2)
12 | |     ($T:ident, #[$attr:meta]) => (
13 |           doc_comment! {
   |  _________-
14 |               concat!("The smallest value that can be represented by this integer type.
15 |   Use [`", stringify!($T), "::MIN", "`](../../std/primitive.", stringify!($T), ".html#associatedconstant.MIN) instead.
...
...
28 |               pub const MIN: $T = $T::MIN;
   | |_________- in this macro invocation (#3)
...
48 | |     )
49 | | }
49 | | }
   | | -
   | |_|
   | |_in this expansion of `int_module!` (#1)
   |   in this expansion of `int_module!` (#2)
  ::: library/core/src/num/shells/i16.rs:10:1
   |
   |
10 |   int_module! { i16 }
   |   ------------------- in this macro invocation (#1)

error: unexpected token: `(/*ERROR*/)`
   |
3  | / macro_rules! doc_comment {
3  | / macro_rules! doc_comment {
4  | |     ($x:expr, $($tt:tt)*) => {
5  | |         #[doc = $x]
   | |                 ^^
6  | |         $($tt)*
8  | | }
8  | | }
   | |_- in this expansion of `doc_comment!` (#3)
10 |   macro_rules! int_module {
   |  _-
   | |_|
   | |
   | |
11 | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   | |                    --------------------------------------------------------------- in this macro invocation (#2)
12 | |     ($T:ident, #[$attr:meta]) => (
13 | |         doc_comment! {
31 | /         doc_comment! {
31 | /         doc_comment! {
32 |               concat!("The largest value that can be represented by this integer type.
33 |   Use [`", stringify!($T), "::MAX", "`](../../std/primitive.", stringify!($T), ".html#associatedconstant.MAX) instead.
...
...
46 |               pub const MAX: $T = $T::MAX;
   | |_________- in this macro invocation (#3)
48 | |     )
49 | | }
   | | -
   | | -
   | |_|
   | |_in this expansion of `int_module!` (#1)
   |   in this expansion of `int_module!` (#2)
  ::: library/core/src/num/shells/i16.rs:10:1
   |
   |
10 |   int_module! { i16 }
   |   ------------------- in this macro invocation (#1)

error: unexpected token: `(/*ERROR*/)`
   |
3  | / macro_rules! doc_comment {
3  | / macro_rules! doc_comment {
4  | |     ($x:expr, $($tt:tt)*) => {
5  | |         #[doc = $x]
   | |                 ^^
6  | |         $($tt)*
8  | | }
8  | | }
   | |_- in this expansion of `doc_comment!` (#3)
10 |   macro_rules! int_module {
   |  _-
   | |_|
   | |
   | |
11 | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   | |                    --------------------------------------------------------------- in this macro invocation (#2)
12 | |     ($T:ident, #[$attr:meta]) => (
13 |           doc_comment! {
   |  _________-
14 |               concat!("The smallest value that can be represented by this integer type.
15 |   Use [`", stringify!($T), "::MIN", "`](../../std/primitive.", stringify!($T), ".html#associatedconstant.MIN) instead.
...
...
28 |               pub const MIN: $T = $T::MIN;
   | |_________- in this macro invocation (#3)
...
48 | |     )
49 | | }
49 | | }
   | | -
   | |_|
   | |_in this expansion of `int_module!` (#1)
   |   in this expansion of `int_module!` (#2)
  ::: library/core/src/num/shells/i32.rs:10:1
   |
   |
10 |   int_module! { i32 }
   |   ------------------- in this macro invocation (#1)

error: unexpected token: `(/*ERROR*/)`
   |
3  | / macro_rules! doc_comment {
3  | / macro_rules! doc_comment {
4  | |     ($x:expr, $($tt:tt)*) => {
5  | |         #[doc = $x]
   | |                 ^^
6  | |         $($tt)*
8  | | }
8  | | }
   | |_- in this expansion of `doc_comment!` (#3)
10 |   macro_rules! int_module {
   |  _-
   | |_|
   | |
   | |
11 | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   | |                    --------------------------------------------------------------- in this macro invocation (#2)
12 | |     ($T:ident, #[$attr:meta]) => (
13 | |         doc_comment! {
31 | /         doc_comment! {
31 | /         doc_comment! {
32 |               concat!("The largest value that can be represented by this integer type.
33 |   Use [`", stringify!($T), "::MAX", "`](../../std/primitive.", stringify!($T), ".html#associatedconstant.MAX) instead.
...
...
46 |               pub const MAX: $T = $T::MAX;
   | |_________- in this macro invocation (#3)
48 | |     )
49 | | }
   | | -
   | | -
   | |_|
   | |_in this expansion of `int_module!` (#1)
   |   in this expansion of `int_module!` (#2)
  ::: library/core/src/num/shells/i32.rs:10:1
   |
   |
10 |   int_module! { i32 }
   |   ------------------- in this macro invocation (#1)

error: unexpected token: `(/*ERROR*/)`
   |
3  | / macro_rules! doc_comment {
3  | / macro_rules! doc_comment {
4  | |     ($x:expr, $($tt:tt)*) => {
5  | |         #[doc = $x]
   | |                 ^^
6  | |         $($tt)*
8  | | }
8  | | }
   | |_- in this expansion of `doc_comment!` (#3)
10 |   macro_rules! int_module {
   |  _-
   | |_|
   | |
   | |
11 | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   | |                    --------------------------------------------------------------- in this macro invocation (#2)
12 | |     ($T:ident, #[$attr:meta]) => (
13 |           doc_comment! {
   |  _________-
14 |               concat!("The smallest value that can be represented by this integer type.
15 |   Use [`", stringify!($T), "::MIN", "`](../../std/primitive.", stringify!($T), ".html#associatedconstant.MIN) instead.
...
...
28 |               pub const MIN: $T = $T::MIN;
   | |_________- in this macro invocation (#3)
...
48 | |     )
49 | | }
49 | | }
   | | -
   | |_|
   | |_in this expansion of `int_module!` (#1)
   |   in this expansion of `int_module!` (#2)
  ::: library/core/src/num/shells/i64.rs:10:1
   |
   |
10 |   int_module! { i64 }
   |   ------------------- in this macro invocation (#1)

error: unexpected token: `(/*ERROR*/)`
   |
3  | / macro_rules! doc_comment {
3  | / macro_rules! doc_comment {
4  | |     ($x:expr, $($tt:tt)*) => {
5  | |         #[doc = $x]
   | |                 ^^
6  | |         $($tt)*
8  | | }
8  | | }
   | |_- in this expansion of `doc_comment!` (#3)
10 |   macro_rules! int_module {
   |  _-
   | |_|
   | |
   | |
11 | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   | |                    --------------------------------------------------------------- in this macro invocation (#2)
12 | |     ($T:ident, #[$attr:meta]) => (
13 | |         doc_comment! {
31 | /         doc_comment! {
31 | /         doc_comment! {
32 |               concat!("The largest value that can be represented by this integer type.
33 |   Use [`", stringify!($T), "::MAX", "`](../../std/primitive.", stringify!($T), ".html#associatedconstant.MAX) instead.
...
...
46 |               pub const MAX: $T = $T::MAX;
   | |_________- in this macro invocation (#3)
48 | |     )
49 | | }
   | | -
   | | -
   | |_|
   | |_in this expansion of `int_module!` (#1)
   |   in this expansion of `int_module!` (#2)
  ::: library/core/src/num/shells/i64.rs:10:1
   |
   |
10 |   int_module! { i64 }
   |   ------------------- in this macro invocation (#1)

error: unexpected token: `(/*ERROR*/)`
   |
3  | / macro_rules! doc_comment {
3  | / macro_rules! doc_comment {
4  | |     ($x:expr, $($tt:tt)*) => {
5  | |         #[doc = $x]
   | |                 ^^
6  | |         $($tt)*
8  | | }
8  | | }
   | |_- in this expansion of `doc_comment!` (#3)
10 |   macro_rules! int_module {
   |  _-
   | |_|
   | |
   | |
11 | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   | |                    --------------------------------------------------------------- in this macro invocation (#2)
12 | |     ($T:ident, #[$attr:meta]) => (
13 |           doc_comment! {
   |  _________-
14 |               concat!("The smallest value that can be represented by this integer type.
15 |   Use [`", stringify!($T), "::MIN", "`](../../std/primitive.", stringify!($T), ".html#associatedconstant.MIN) instead.
...
...
28 |               pub const MIN: $T = $T::MIN;
   | |_________- in this macro invocation (#3)
...
48 | |     )
49 | | }
49 | | }
   | | -
   | |_|
   | |_in this expansion of `int_module!` (#1)
   |   in this expansion of `int_module!` (#2)
  ::: library/core/src/num/shells/i8.rs:10:1
   |
   |
10 |   int_module! { i8 }
   |   ------------------ in this macro invocation (#1)

error: unexpected token: `(/*ERROR*/)`
   |
3  | / macro_rules! doc_comment {
3  | / macro_rules! doc_comment {
4  | |     ($x:expr, $($tt:tt)*) => {
5  | |         #[doc = $x]
   | |                 ^^
6  | |         $($tt)*
8  | | }
8  | | }
   | |_- in this expansion of `doc_comment!` (#3)
10 |   macro_rules! int_module {
   |  _-
   | |_|
   | |
   | |
11 | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   | |                    --------------------------------------------------------------- in this macro invocation (#2)
12 | |     ($T:ident, #[$attr:meta]) => (
13 | |         doc_comment! {
31 | /         doc_comment! {
31 | /         doc_comment! {
32 |               concat!("The largest value that can be represented by this integer type.
33 |   Use [`", stringify!($T), "::MAX", "`](../../std/primitive.", stringify!($T), ".html#associatedconstant.MAX) instead.
...
...
46 |               pub const MAX: $T = $T::MAX;
   | |_________- in this macro invocation (#3)
48 | |     )
49 | | }
   | | -
   | | -
   | |_|
   | |_in this expansion of `int_module!` (#1)
   |   in this expansion of `int_module!` (#2)
  ::: library/core/src/num/shells/i8.rs:10:1
   |
   |
10 |   int_module! { i8 }
   |   ------------------ in this macro invocation (#1)

error: unexpected token: `(/*ERROR*/)`
   |
3  | / macro_rules! doc_comment {
3  | / macro_rules! doc_comment {
4  | |     ($x:expr, $($tt:tt)*) => {
5  | |         #[doc = $x]
   | |                 ^^
6  | |         $($tt)*
8  | | }
8  | | }
   | |_- in this expansion of `doc_comment!` (#3)
10 |   macro_rules! int_module {
   |  _-
   | |_|
   | |
   | |
11 | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   | |                    --------------------------------------------------------------- in this macro invocation (#2)
12 | |     ($T:ident, #[$attr:meta]) => (
13 |           doc_comment! {
   |  _________-
14 |               concat!("The smallest value that can be represented by this integer type.
15 |   Use [`", stringify!($T), "::MIN", "`](../../std/primitive.", stringify!($T), ".html#associatedconstant.MIN) instead.
...
...
28 |               pub const MIN: $T = $T::MIN;
   | |_________- in this macro invocation (#3)
...
48 | |     )
49 | | }
49 | | }
   | | -
   | |_|
   | |_in this expansion of `int_module!` (#1)
   |   in this expansion of `int_module!` (#2)
  ::: library/core/src/num/shells/isize.rs:10:1
   |
   |
10 |   int_module! { isize }
   |   --------------------- in this macro invocation (#1)

error: unexpected token: `(/*ERROR*/)`
   |
3  | / macro_rules! doc_comment {
3  | / macro_rules! doc_comment {
4  | |     ($x:expr, $($tt:tt)*) => {
5  | |         #[doc = $x]
   | |                 ^^
6  | |         $($tt)*
8  | | }
8  | | }
   | |_- in this expansion of `doc_comment!` (#3)
10 |   macro_rules! int_module {
   |  _-
   | |_|
   | |
   | |
11 | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
   | |                    --------------------------------------------------------------- in this macro invocation (#2)
12 | |     ($T:ident, #[$attr:meta]) => (
13 | |         doc_comment! {
31 | /         doc_comment! {
31 | /         doc_comment! {
32 |               concat!("The largest value that can be represented by this integer type.
33 |   Use [`", stringify!($T), "::MAX", "`](../../std/primitive.", stringify!($T), ".html#associatedconstant.MAX) instead.
...
...
46 |               pub const MAX: $T = $T::MAX;
   | |_________- in this macro invocation (#3)
48 | |     )
49 | | }
   | | -
   | | -
   | |_|
   | |_in this expansion of `int_module!` (#1)
   |   in this expansion of `int_module!` (#2)
  ::: library/core/src/num/shells/isize.rs:10:1
   |
   |
10 |   int_module! { isize }
   |   --------------------- in this macro invocation (#1)

error: unexpected token: `(/*ERROR*/)`
   |
3  | / macro_rules! doc_comment {
3  | / macro_rules! doc_comment {
4  | |     ($x:expr, $($tt:tt)*) => {
5  | |         #[doc = $x]
   | |                 ^^
6  | |         $($tt)*
8  | | }
8  | | }
   | |_- in this expansion of `doc_comment!` (#2)
10 | / macro_rules! int_module {
10 | / macro_rules! int_module {
11 | |     ($T:ident) => (int_module!($T, #[stable(feature = "rust1", since = "1.0.0")]););
12 | |     ($T:ident, #[$attr:meta]) => (
13 |           doc_comment! {
   |  _________-
14 |               concat!("The smallest value that can be represented by this integer type.

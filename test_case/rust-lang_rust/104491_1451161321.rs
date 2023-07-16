plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:0fdabd83e1d3faaa8e9cfd7c00031e3a92997344)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.87
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: unreachable block in `if` or `while` expression
    |
29  | /   macro_rules! iterator {
30  | |       (
30  | |       (
31  | |           struct $name:ident -> $ptr:ty,
32  | |           $elem:ty,
404 | |                       if is_empty!(self) {
    | |  ________________________________________^
405 | | |                         None
406 | | |                     } else {
406 | | |                     } else {
    | | |_____________________^ unreachable block in `if` or `while` expression
665 | |       }
666 | |   }
    | |___- in this expansion of `iterator!`
    |
    |
   ::: library/core/src/panic.rs:57:9
    |
57  |             $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
    |             ---------------------------------------------------------------- any code following this expression is unreachable
    |
   ::: library/core/src/slice/iter.rs:133:1
    |
133 |   / iterator! {struct Iter -> *const T, &'a T, const, {/* no mut */}, {
134 |   |     // FIXME(const_trait_impl)
135 |   |     // fn is_sorted_by<F>(self, mut compare: F) -> bool
136 |   |     // where
143 |   |     // }
144 |   | }}
    |   |__- in this macro invocation
    |
    |
    = note: `-D unreachable-code` implied by `-D warnings`

error: unreachable block in `if` or `while` expression
    |
29  | /   macro_rules! iterator {
30  | |       (
30  | |       (
31  | |           struct $name:ident -> $ptr:ty,
32  | |           $elem:ty,
620 | |                       if is_empty!(self) {
    | |  ________________________________________^
621 | | |                         None
622 | | |                     } else {
622 | | |                     } else {
    | | |_____________________^ unreachable block in `if` or `while` expression
665 | |       }
666 | |   }
    | |___- in this expansion of `iterator!`
    |
    |
   ::: library/core/src/panic.rs:57:9
    |
57  |             $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
    |             ---------------------------------------------------------------- any code following this expression is unreachable
    |
   ::: library/core/src/slice/iter.rs:133:1
    |
133 |   / iterator! {struct Iter -> *const T, &'a T, const, {/* no mut */}, {
134 |   |     // FIXME(const_trait_impl)
135 |   |     // fn is_sorted_by<F>(self, mut compare: F) -> bool
136 |   |     // where
143 |   |     // }
144 |   | }}
    |   |__- in this macro invocation


error: unreachable block in `if` or `while` expression
   ::: library/core/src/panic.rs:57:9
    |
57  |            $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
    |            ---------------------------------------------------------------- any code following this expression is unreachable
    |            ---------------------------------------------------------------- any code following this expression is unreachable
    |
   ::: library/core/src/slice/iter.rs:366:1
    |
366 |    iterator! {struct IterMut -> *mut T, &'a mut T, mut, {mut}, {}}
   --> library/core/src/slice/iter/macros.rs:404:40
    |
29  | /  macro_rules! iterator {
30  | |      (
30  | |      (
31  | |          struct $name:ident -> $ptr:ty,
32  | |          $elem:ty,
404 | |                      if is_empty!(self) {
    | | ________________________________________^
405 | ||                         None
406 | ||                     } else {
406 | ||                     } else {
    | ||_____________________^ unreachable block in `if` or `while` expression
665 | |      }
666 | |  }
    | |__- in this expansion of `iterator!`


error: unreachable block in `if` or `while` expression
   ::: library/core/src/panic.rs:57:9
    |
57  |            $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
    |            ---------------------------------------------------------------- any code following this expression is unreachable
    |            ---------------------------------------------------------------- any code following this expression is unreachable
    |
   ::: library/core/src/slice/iter.rs:366:1
    |
366 |    iterator! {struct IterMut -> *mut T, &'a mut T, mut, {mut}, {}}
   --> library/core/src/slice/iter/macros.rs:620:40
    |
29  | /  macro_rules! iterator {
30  | |      (
30  | |      (
31  | |          struct $name:ident -> $ptr:ty,
32  | |          $elem:ty,
620 | |                      if is_empty!(self) {
    | | ________________________________________^
621 | ||                         None
622 | ||                     } else {
622 | ||                     } else {
    | ||_____________________^ unreachable block in `if` or `while` expression
665 | |      }
666 | |  }
    | |__- in this expansion of `iterator!`

---
    | |__- in this expansion of `$crate::panic!` (#4)
...
771 |  / macro_rules! todo {
772 |  |     () => {
773 |  |         $crate::panicking::panic("not yet implemented")
775 |  |     ($($arg:tt)+) => {
775 |  |     ($($arg:tt)+) => {
776 |  |         $crate::panic!("not yet implemented: {}", $crate::format_args!($($arg)+))
    |  |         |
    |  |         in this macro invocation (#4)
    |  |         in this macro invocation (#5)
777 |  |     };
777 |  |     };
778 |  | }
    |  |_- in this expansion of `todo!` (#3)
891 |  /     macro_rules! const_format_args {
891 |  /     macro_rules! const_format_args {
892 |  |         ($fmt:expr) => {{ /* compiler built-in */ }};
893 |  |         ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
    |  |_____- in this expansion of `$crate::const_format_args!` (#6)
    |
   ::: library/core/src/slice/iter/macros.rs:12:1
    |
    |
12  |  / macro_rules! len {
13  |  |     ($self: ident) => {{
14  |  |         #![allow(unused_unsafe)] // we're sometimes used within an unsafe block
...    |
...    |
18  |  |             todo!("blocked on #92512")
...    |
25  |  |     }};
26  |  | }
26  |  | }
    |  |_- in this expansion of `len!` (#2)
29  |  / macro_rules! iterator {
30  |  |     (
30  |  |     (
31  |  |         struct $name:ident -> $ptr:ty,
32  |  |         $elem:ty,
...    |
66  |  |                 unsafe { from_raw_parts(self.ptr.as_ptr(), len!(self)) }
...    |
665 |  |     }
666 |  | }
    |  |_- in this expansion of `iterator!` (#1)
---
    |  |_- in this expansion of `$crate::panic::panic_2021!` (#5)
    |
   ::: library/core/src/slice/iter.rs:133:1
    |
133 |  / iterator! {struct Iter -> *const T, &'a T, const, {/* no mut */}, {
134 |  |     // FIXME(const_trait_impl)
135 |  |     // fn is_sorted_by<F>(self, mut compare: F) -> bool
136 |  |     // where
143 |  |     // }
144 |  | }}
    |  |__- in this macro invocation (#1)
    |
    |
    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
error[E0015]: cannot call non-const formatting macro in constant functions
   --> library/core/src/macros/mod.rs:776:51
    |
7   | /  macro_rules! panic {
---
    | |__- in this expansion of `$crate::panic!` (#4)
...
771 |  / macro_rules! todo {
772 |  |     () => {
773 |  |         $crate::panicking::panic("not yet implemented")
775 |  |     ($($arg:tt)+) => {
775 |  |     ($($arg:tt)+) => {
776 |  |         $crate::panic!("not yet implemented: {}", $crate::format_args!($($arg)+))
    |  |         |
    |  |         in this macro invocation (#4)
    |  |         in this macro invocation (#5)
777 |  |     };
777 |  |     };
778 |  | }
    |  |_- in this expansion of `todo!` (#3)
891 |  /     macro_rules! const_format_args {
891 |  /     macro_rules! const_format_args {
892 |  |         ($fmt:expr) => {{ /* compiler built-in */ }};
893 |  |         ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
    |  |_____- in this expansion of `$crate::const_format_args!` (#6)
    |
   ::: library/core/src/slice/iter/macros.rs:12:1
    |
    |
12  |  / macro_rules! len {
13  |  |     ($self: ident) => {{
14  |  |         #![allow(unused_unsafe)] // we're sometimes used within an unsafe block
...    |
...    |
18  |  |             todo!("blocked on #92512")
...    |
25  |  |     }};
26  |  | }
26  |  | }
    |  |_- in this expansion of `len!` (#2)
29  |  / macro_rules! iterator {
30  |  |     (
30  |  |     (
31  |  |         struct $name:ident -> $ptr:ty,
32  |  |         $elem:ty,
...    |
66  |  |                 unsafe { from_raw_parts(self.ptr.as_ptr(), len!(self)) }
...    |
665 |  |     }
666 |  | }
    |  |_- in this expansion of `iterator!` (#1)
    |  |_- in this expansion of `iterator!` (#1)
    |
   ::: library/core/src/slice/iter.rs:366:1
    |
366 |    iterator! {struct IterMut -> *mut T, &'a mut T, mut, {mut}, {}}
    |
   ::: library/core/src/panic.rs:48:1
    |
48  |  / pub macro panic_2021 {
---
58  |  |     ),
59  |  | }
    |  |_- in this expansion of `$crate::panic::panic_2021!` (#5)
    |
    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
For more information about this error, try `rustc --explain E0015`.
error: could not compile `core` due to 6 previous errors
Build completed unsuccessfully in 0:00:27

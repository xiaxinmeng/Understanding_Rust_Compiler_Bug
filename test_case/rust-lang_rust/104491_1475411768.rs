plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3fb2b44a4eaebb9ed8086446bde46c27199ef5ed)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.87
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0636]: the feature `const_pointer_byte_offsets` has already been declared
    |
147 | #![feature(const_pointer_byte_offsets)]
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^


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


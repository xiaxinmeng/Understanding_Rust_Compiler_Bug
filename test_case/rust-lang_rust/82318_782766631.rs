plain
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  EXTRA_VARIABLES: {
 "CI_ONLY_WHEN_SUBMODULES_CHANGED": 1
##[endgroup]
adding extra environment variable CI_ONLY_WHEN_SUBMODULES_CHANGED
linux builder detected, using docker to run the build
##[group]Run src/ci/scripts/should-skip-this.sh
---
normalized stderr:


expected stderr:
error: Question mark operator is useless here
  --> $DIR/needless_question_mark.rs:23:12
   |
LL |     return Some(to.magic?);
   |            ^^^^^^^^^^^^^^^ help: try: `to.magic`
   |
   = note: `-D clippy::needless-question-mark` implied by `-D warnings`

error: Question mark operator is useless here
  --> $DIR/needless_question_mark.rs:31:12
   |
LL |     return Some(to.magic?)
   |            ^^^^^^^^^^^^^^^ help: try: `to.magic`

error: Question mark operator is useless here
  --> $DIR/needless_question_mark.rs:36:5
   |
LL |     Some(to.magic?)
   |     ^^^^^^^^^^^^^^^ help: try: `to.magic`

error: Question mark operator is useless here
  --> $DIR/needless_question_mark.rs:41:21
   |
LL |     to.and_then(|t| Some(t.magic?))
   |                     ^^^^^^^^^^^^^^ help: try: `t.magic`

error: Question mark operator is useless here
  --> $DIR/needless_question_mark.rs:50:9
   |
LL |         Some(t.magic?)
   |         ^^^^^^^^^^^^^^ help: try: `t.magic`

error: Question mark operator is useless here
  --> $DIR/needless_question_mark.rs:55:12
   |
LL |     return Ok(tr.magic?);
   |            ^^^^^^^^^^^^^ help: try: `tr.magic`

error: Question mark operator is useless here
  --> $DIR/needless_question_mark.rs:62:12
   |
LL |     return Ok(tr.magic?)
   |            ^^^^^^^^^^^^^ help: try: `tr.magic`

error: Question mark operator is useless here
  --> $DIR/needless_question_mark.rs:66:5
   |
LL |     Ok(tr.magic?)
   |     ^^^^^^^^^^^^^ help: try: `tr.magic`

error: Question mark operator is useless here
  --> $DIR/needless_question_mark.rs:70:21
   |
LL |     tr.and_then(|t| Ok(t.magic?))
   |                     ^^^^^^^^^^^^ help: try: `t.magic`

error: Question mark operator is useless here
  --> $DIR/needless_question_mark.rs:78:9
   |
LL |         Ok(t.magic?)
   |         ^^^^^^^^^^^^ help: try: `t.magic`

error: Question mark operator is useless here
  --> $DIR/needless_question_mark.rs:85:16
   |
LL |         return Ok(t.magic?);
   |                ^^^^^^^^^^^^ help: try: `t.magic`

error: Question mark operator is useless here
  --> $DIR/needless_question_mark.rs:138:9
   |
LL |         Ok(to.magic?) // should be triggered
   |         ^^^^^^^^^^^^^ help: try: `to.magic`

error: Question mark operator is useless here
  --> $DIR/needless_question_mark.rs:154:9
   |
LL |         Some(to.magic?) // should be triggered
   |         ^^^^^^^^^^^^^^^ help: try: `to.magic`

error: Question mark operator is useless here
  --> $DIR/needless_question_mark.rs:162:9
   |
LL |         Ok(to.magic?) // should be triggered
   |         ^^^^^^^^^^^^^ help: try: `to.magic`
error: aborting due to 14 previous errors




diff of stderr:

-error: Question mark operator is useless here
-  --> $DIR/needless_question_mark.rs:23:12
-   |
-LL |     return Some(to.magic?);
-   |            ^^^^^^^^^^^^^^^ help: try: `to.magic`
-   |
-   = note: `-D clippy::needless-question-mark` implied by `-D warnings`
-
-error: Question mark operator is useless here
-  --> $DIR/needless_question_mark.rs:31:12
-   |
-LL |     return Some(to.magic?)
-   |            ^^^^^^^^^^^^^^^ help: try: `to.magic`
-
-error: Question mark operator is useless here
-  --> $DIR/needless_question_mark.rs:36:5
-   |
-LL |     Some(to.magic?)
-   |     ^^^^^^^^^^^^^^^ help: try: `to.magic`
-
-error: Question mark operator is useless here
-  --> $DIR/needless_question_mark.rs:41:21
-   |
-LL |     to.and_then(|t| Some(t.magic?))
-   |                     ^^^^^^^^^^^^^^ help: try: `t.magic`
-
-error: Question mark operator is useless here
-  --> $DIR/needless_question_mark.rs:50:9
-   |
-LL |         Some(t.magic?)
-   |         ^^^^^^^^^^^^^^ help: try: `t.magic`
-
-error: Question mark operator is useless here
-  --> $DIR/needless_question_mark.rs:55:12
-   |
-LL |     return Ok(tr.magic?);
-   |            ^^^^^^^^^^^^^ help: try: `tr.magic`
-
-error: Question mark operator is useless here
-  --> $DIR/needless_question_mark.rs:62:12
-   |
-LL |     return Ok(tr.magic?)
-   |            ^^^^^^^^^^^^^ help: try: `tr.magic`
-
-error: Question mark operator is useless here
-  --> $DIR/needless_question_mark.rs:66:5
-   |
-LL |     Ok(tr.magic?)
-   |     ^^^^^^^^^^^^^ help: try: `tr.magic`
-
-error: Question mark operator is useless here
-  --> $DIR/needless_question_mark.rs:70:21
-   |
-LL |     tr.and_then(|t| Ok(t.magic?))
-   |                     ^^^^^^^^^^^^ help: try: `t.magic`
-
-error: Question mark operator is useless here
-  --> $DIR/needless_question_mark.rs:78:9
-   |
-LL |         Ok(t.magic?)
-   |         ^^^^^^^^^^^^ help: try: `t.magic`
error: test failed, to rerun pass '--test compile-test'
error: test failed, to rerun pass '--test compile-test'
-error: Question mark operator is useless here
-  --> $DIR/needless_question_mark.rs:85:16
-   |
-LL |         return Ok(t.magic?);
-   |                ^^^^^^^^^^^^ help: try: `t.magic`
-
-error: Question mark operator is useless here
-  --> $DIR/needless_question_mark.rs:138:9
-   |
-LL |         Ok(to.magic?) // should be triggered
-   |         ^^^^^^^^^^^^^ help: try: `to.magic`
-
-error: Question mark operator is useless here
-  --> $DIR/needless_question_mark.rs:154:9
-   |
-LL |         Some(to.magic?) // should be triggered
-   |         ^^^^^^^^^^^^^^^ help: try: `to.magic`
-
-error: Question mark operator is useless here
-  --> $DIR/needless_question_mark.rs:162:9
-   |
-LL |         Ok(to.magic?) // should be triggered
-   |         ^^^^^^^^^^^^^ help: try: `to.magic`
-error: aborting due to 14 previous errors
-
-


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-1989c91f44b16c80/out/test_build_base/needless_question_mark.stderr
// run-rustfix


#![warn(clippy::needless_question_mark)]
#![allow(
    clippy::needless_return,
    clippy::unnecessary_unwrap,
    clippy::upper_case_acronyms,
    unused_must_use
)]
)]
#![feature(custom_inner_attributes)]
struct TO {
struct TO {
    magic: Option<usize>,

struct TR {
struct TR {
    magic: Result<usize, bool>,


fn simple_option_bad1(to: TO) -> Option<usize> {
    // return as a statement
    return Some(to.magic?);


// formatting will add a semi-colon, which would make
// this identical to the test case above
#[rustfmt::skip]
fn simple_option_bad2(to: TO) -> Option<usize> {
    // return as an expression
    return Some(to.magic?)


fn simple_option_bad3(to: TO) -> Option<usize> {
    // block value "return"
    Some(to.magic?)


fn simple_option_bad4(to: Option<TO>) -> Option<usize> {
    // single line closure
    to.and_then(|t| Some(t.magic?))


// formatting this will remove the block brackets, making
// this test identical to the one above
#[rustfmt::skip]
fn simple_option_bad5(to: Option<TO>) -> Option<usize> {
    // closure with body
    to.and_then(|t| {
        Some(t.magic?)
}


fn simple_result_bad1(tr: TR) -> Result<usize, bool> {
    return Ok(tr.magic?);


// formatting will add a semi-colon, which would make
// this identical to the test case above
#[rustfmt::skip]
fn simple_result_bad2(tr: TR) -> Result<usize, bool> {
    return Ok(tr.magic?)


fn simple_result_bad3(tr: TR) -> Result<usize, bool> {
    Ok(tr.magic?)


fn simple_result_bad4(tr: Result<TR, bool>) -> Result<usize, bool> {
    tr.and_then(|t| Ok(t.magic?))


// formatting this will remove the block brackets, making
// this test identical to the one above
#[rustfmt::skip]
fn simple_result_bad5(tr: Result<TR, bool>) -> Result<usize, bool> {
    tr.and_then(|t| {
        Ok(t.magic?)
}


fn also_bad(tr: Result<TR, bool>) -> Result<usize, bool> {
    if tr.is_ok() {
        let t = tr.unwrap();
        return Ok(t.magic?);
    Err(false)
}


fn false_positive_test<U, T>(x: Result<(), U>) -> Result<(), T>
where
    T: From<U>,
    Ok(x?)
}

fn main() {}
fn main() {}

mod question_mark_none {
    #![clippy::msrv = "1.12.0"]
    fn needless_question_mark_option() -> Option<usize> {
        struct TO {
            magic: Option<usize>,
        }
        let to = TO { magic: None };
        Some(to.magic?) // should not be triggered


    fn needless_question_mark_result() -> Result<usize, bool> {
        struct TO {
            magic: Result<usize, bool>,
        }
        let to = TO { magic: Ok(1_usize) };
        Ok(to.magic?) // should not be triggered

    fn main() {
        needless_question_mark_option();
        needless_question_mark_option();
        needless_question_mark_result();
}

mod question_mark_result {
mod question_mark_result {
    #![clippy::msrv = "1.21.0"]
    fn needless_question_mark_option() -> Option<usize> {
        struct TO {
            magic: Option<usize>,
        }
        let to = TO { magic: None };
        Some(to.magic?) // should not be triggered


    fn needless_question_mark_result() -> Result<usize, bool> {
        struct TO {
            magic: Result<usize, bool>,
        }
        let to = TO { magic: Ok(1_usize) };
        Ok(to.magic?) // should be triggered

    fn main() {
        needless_question_mark_option();
        needless_question_mark_option();
        needless_question_mark_result();
}


mod question_mark_both {
    #![clippy::msrv = "1.22.0"]
    fn needless_question_mark_option() -> Option<usize> {
        struct TO {
            magic: Option<usize>,
        }
        let to = TO { magic: None };
        Some(to.magic?) // should be triggered


    fn needless_question_mark_result() -> Result<usize, bool> {
        struct TO {
            magic: Result<usize, bool>,
        }
        let to = TO { magic: Ok(1_usize) };
        Ok(to.magic?) // should be triggered

    fn main() {
        needless_question_mark_option();
        needless_question_mark_option();
        needless_question_mark_result();
}


expected fixed:
expected fixed:
// run-rustfix

#![warn(clippy::needless_question_mark)]
#![allow(
    clippy::needless_return,
    clippy::unnecessary_unwrap,
    clippy::upper_case_acronyms,
    unused_must_use
)]
)]
#![feature(custom_inner_attributes)]
struct TO {
struct TO {
    magic: Option<usize>,

struct TR {
struct TR {
    magic: Result<usize, bool>,


fn simple_option_bad1(to: TO) -> Option<usize> {
    // return as a statement
    return to.magic;


// formatting will add a semi-colon, which would make
// this identical to the test case above
#[rustfmt::skip]
fn simple_option_bad2(to: TO) -> Option<usize> {
    // return as an expression
    return to.magic


fn simple_option_bad3(to: TO) -> Option<usize> {
    // block value "return"
    to.magic


fn simple_option_bad4(to: Option<TO>) -> Option<usize> {
    // single line closure
    to.and_then(|t| t.magic)


// formatting this will remove the block brackets, making
// this test identical to the one above
#[rustfmt::skip]
fn simple_option_bad5(to: Option<TO>) -> Option<usize> {
    // closure with body
    to.and_then(|t| {
        t.magic
}


fn simple_result_bad1(tr: TR) -> Result<usize, bool> {
    return tr.magic;


// formatting will add a semi-colon, which would make
// this identical to the test case above
#[rustfmt::skip]
fn simple_result_bad2(tr: TR) -> Result<usize, bool> {
    return tr.magic


fn simple_result_bad3(tr: TR) -> Result<usize, bool> {
    tr.magic


fn simple_result_bad4(tr: Result<TR, bool>) -> Result<usize, bool> {
    tr.and_then(|t| t.magic)


// formatting this will remove the block brackets, making
// this test identical to the one above
#[rustfmt::skip]
fn simple_result_bad5(tr: Result<TR, bool>) -> Result<usize, bool> {
    tr.and_then(|t| {
        t.magic
}


fn also_bad(tr: Result<TR, bool>) -> Result<usize, bool> {
    if tr.is_ok() {
        let t = tr.unwrap();
        return t.magic;
    Err(false)
}


fn false_positive_test<U, T>(x: Result<(), U>) -> Result<(), T>
where
    T: From<U>,
    Ok(x?)
}

fn main() {}
fn main() {}

mod question_mark_none {
    #![clippy::msrv = "1.12.0"]
    fn needless_question_mark_option() -> Option<usize> {
        struct TO {
            magic: Option<usize>,
        }
        let to = TO { magic: None };
        Some(to.magic?) // should not be triggered


    fn needless_question_mark_result() -> Result<usize, bool> {
        struct TO {
            magic: Result<usize, bool>,
        }
        let to = TO { magic: Ok(1_usize) };
        Ok(to.magic?) // should not be triggered

    fn main() {
        needless_question_mark_option();
        needless_question_mark_option();
        needless_question_mark_result();
}

mod question_mark_result {
mod question_mark_result {
    #![clippy::msrv = "1.21.0"]
    fn needless_question_mark_option() -> Option<usize> {
        struct TO {
            magic: Option<usize>,
        }
        let to = TO { magic: None };
        Some(to.magic?) // should not be triggered


    fn needless_question_mark_result() -> Result<usize, bool> {
        struct TO {
            magic: Result<usize, bool>,
        }
        let to = TO { magic: Ok(1_usize) };
        to.magic // should be triggered
---
normalized stderr:


expected stderr:
error: returning an `Err(_)` with the `?` operator
  --> $DIR/try_err.rs:19:9
LL |         Err(err)?;
LL |         Err(err)?;
   |         ^^^^^^^^^ help: try this: `return Err(err)`
note: the lint level is defined here
  --> $DIR/try_err.rs:4:9
   |
   |
LL | #![deny(clippy::try_err)]


error: returning an `Err(_)` with the `?` operator
  --> $DIR/try_err.rs:29:9
LL |         Err(err)?;
LL |         Err(err)?;
   |         ^^^^^^^^^ help: try this: `return Err(err.into())`

error: returning an `Err(_)` with the `?` operator
  --> $DIR/try_err.rs:49:17
LL |                 Err(err)?;
LL |                 Err(err)?;
   |                 ^^^^^^^^^ help: try this: `return Err(err)`

error: returning an `Err(_)` with the `?` operator
  --> $DIR/try_err.rs:68:17
LL |                 Err(err)?;
LL |                 Err(err)?;
   |                 ^^^^^^^^^ help: try this: `return Err(err.into())`

error: returning an `Err(_)` with the `?` operator
  --> $DIR/try_err.rs:87:23
   |
LL |             Err(_) => Err(1)?,
   |                       ^^^^^^^ help: try this: `return Err(1)`
...
LL |     try_validation!(Ok::<_, i32>(5));
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: returning an `Err(_)` with the `?` operator
  --> $DIR/try_err.rs:102:23
   |
LL |             Err(_) => Err(ret_one!())?,
   |                       ^^^^^^^^^^^^^^^^ help: try this: `return Err(ret_one!())`
...
LL |     try_validation_in_macro!(Ok::<_, i32>(5));
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: returning an `Err(_)` with the `?` operator
  --> $DIR/try_err.rs:141:9
   |
LL |         Err(foo!())?;
   |         ^^^^^^^^^^^^ help: try this: `return Err(foo!())`

error: returning an `Err(_)` with the `?` operator
  --> $DIR/try_err.rs:148:9
   |
LL |         Err(io::ErrorKind::WriteZero)?
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `return Poll::Ready(Err(io::ErrorKind::WriteZero.into()))`

error: returning an `Err(_)` with the `?` operator
  --> $DIR/try_err.rs:150:9
   |
LL |         Err(io::Error::new(io::ErrorKind::InvalidInput, "error"))?
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `return Poll::Ready(Err(io::Error::new(io::ErrorKind::InvalidInput, "error")))`

error: returning an `Err(_)` with the `?` operator
  --> $DIR/try_err.rs:158:9
   |
LL |         Err(io::ErrorKind::NotFound)?
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `return Poll::Ready(Some(Err(io::ErrorKind::NotFound.into())))`
error: aborting due to 10 previous errors




diff of stderr:

-error: returning an `Err(_)` with the `?` operator
-  --> $DIR/try_err.rs:19:9
-   |
-LL |         Err(err)?;
-   |         ^^^^^^^^^ help: try this: `return Err(err)`
-   |
-  --> $DIR/try_err.rs:4:9
-   |
-   |
-LL | #![deny(clippy::try_err)]
-   |         ^^^^^^^^^^^^^^^
-
-error: returning an `Err(_)` with the `?` operator
-  --> $DIR/try_err.rs:29:9
-   |
-LL |         Err(err)?;
-   |         ^^^^^^^^^ help: try this: `return Err(err.into())`
-
-error: returning an `Err(_)` with the `?` operator
-  --> $DIR/try_err.rs:49:17
-   |
-LL |                 Err(err)?;
-   |                 ^^^^^^^^^ help: try this: `return Err(err)`
-
-error: returning an `Err(_)` with the `?` operator
-  --> $DIR/try_err.rs:68:17
-   |
-LL |                 Err(err)?;
-   |                 ^^^^^^^^^ help: try this: `return Err(err.into())`
-
-error: returning an `Err(_)` with the `?` operator
-  --> $DIR/try_err.rs:87:23
-   |
-LL |             Err(_) => Err(1)?,
-   |                       ^^^^^^^ help: try this: `return Err(1)`
-...
-LL |     try_validation!(Ok::<_, i32>(5));
-   |     --------------------------------- in this macro invocation
-   |
-   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
-
-error: returning an `Err(_)` with the `?` operator
-  --> $DIR/try_err.rs:102:23
-   |
-LL |             Err(_) => Err(ret_one!())?,
-   |                       ^^^^^^^^^^^^^^^^ help: try this: `return Err(ret_one!())`
-...
-LL |     try_validation_in_macro!(Ok::<_, i32>(5));
-   |     ------------------------------------------ in this macro invocation
-   |
-   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
-
-error: returning an `Err(_)` with the `?` operator
-  --> $DIR/try_err.rs:141:9
-   |
-LL |         Err(foo!())?;
-   |         ^^^^^^^^^^^^ help: try this: `return Err(foo!())`
-
-error: returning an `Err(_)` with the `?` operator
-  --> $DIR/try_err.rs:148:9
-   |
-LL |         Err(io::ErrorKind::WriteZero)?
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `return Poll::Ready(Err(io::ErrorKind::WriteZero.into()))`
-
-error: returning an `Err(_)` with the `?` operator
-  --> $DIR/try_err.rs:150:9
-   |
-LL |         Err(io::Error::new(io::ErrorKind::InvalidInput, "error"))?
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `return Poll::Ready(Err(io::Error::new(io::ErrorKind::InvalidInput, "error")))`
-
-error: returning an `Err(_)` with the `?` operator
-  --> $DIR/try_err.rs:158:9
-   |
-LL |         Err(io::ErrorKind::NotFound)?
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `return Poll::Ready(Some(Err(io::ErrorKind::NotFound.into())))`
-error: aborting due to 10 previous errors
-
-


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-1989c91f44b16c80/out/test_build_base/try_err.stderr
// run-rustfix
// run-rustfix
// aux-build:macro_rules.rs

#![deny(clippy::try_err)]
#![allow(clippy::unnecessary_wraps, clippy::needless_question_mark)]
#[macro_use]
extern crate macro_rules;

use std::io;
use std::io;
use std::task::Poll;

// Tests that a simple case works
// Should flag `Err(err)?`
pub fn basic_test() -> Result<i32, i32> {
    let err: i32 = 1;
    // To avoid warnings during rustfix
        Err(err)?;
    }
    Ok(0)
}
}

// Tests that `.into()` is added when appropriate
pub fn into_test() -> Result<i32, i32> {
    let err: u8 = 1;
    // To avoid warnings during rustfix
        Err(err)?;
    }
    Ok(0)
}
}

// Tests that tries in general don't trigger the error
pub fn negative_test() -> Result<i32, i32> {
    Ok(nested_error()? + 1)


// Tests that `.into()` isn't added when the error type
// matches the surrounding closure's return type, even
// when it doesn't match the surrounding function's.
pub fn closure_matches_test() -> Result<i32, i32> {
    let res: Result<i32, i8> = Some(1)
        .into_iter()
        .map(|i| {
            let err: i8 = 1;
            // To avoid warnings during rustfix
                Err(err)?;
            }
            Ok(i)
        })
        })
        .next()
        .unwrap();

    Ok(res?)
}

// Tests that `.into()` isn't added when the error type
// doesn't match the surrounding closure's return type.
pub fn closure_into_test() -> Result<i32, i32> {
    let res: Result<i32, i16> = Some(1)
        .into_iter()
        .map(|i| {
            let err: i8 = 1;
            // To avoid warnings during rustfix
                Err(err)?;
            }
            Ok(i)
        })
        })
        .next()
        .unwrap();

    Ok(res?)
}

fn nested_error() -> Result<i32, i32> {
    Ok(1)


// Bad suggestion when in macro (see #6242)
macro_rules! try_validation {
    ($e: expr) => {{
        match $e {
            Ok(_) => 0,
            Err(_) => Err(1)?,
    }};
}

macro_rules! ret_one {
macro_rules! ret_one {
    () => {
        1
    };
}

macro_rules! try_validation_in_macro {
    ($e: expr) => {{
        match $e {
            Ok(_) => 0,
            Err(_) => Err(ret_one!())?,
    }};
}


fn calling_macro() -> Result<i32, i32> {
    // macro
    try_validation!(Ok::<_, i32>(5));
    // `Err` arg is another macro
    try_validation_in_macro!(Ok::<_, i32>(5));
    Ok(5)

fn main() {
    basic_test().unwrap();
    into_test().unwrap();
---
    // We don't want to lint in external macros
    try_err!();
}

macro_rules! bar {
    () => {
        String::from("aasdfasdfasdfa")
}

macro_rules! foo {
    () => {
    () => {
        bar!()
    };
}

pub fn macro_inside(fail: bool) -> Result<i32, String> {
    if fail {
        Err(foo!())?;
    Ok(0)
}


pub fn poll_write(n: usize) -> Poll<io::Result<usize>> {
    if n == 0 {
        Err(io::ErrorKind::WriteZero)?
    } else if n == 1 {
        Err(io::Error::new(io::ErrorKind::InvalidInput, "error"))?


    Poll::Ready(Ok(n))


pub fn poll_next(ready: bool) -> Poll<Option<io::Result<()>>> {
    if !ready {
        Err(io::ErrorKind::NotFound)?

    Poll::Ready(None)
}



expected fixed:
// run-rustfix
// aux-build:macro_rules.rs

#![deny(clippy::try_err)]
#![allow(clippy::unnecessary_wraps, clippy::needless_question_mark)]
#[macro_use]
extern crate macro_rules;

use std::io;
use std::io;
use std::task::Poll;

// Tests that a simple case works
// Should flag `Err(err)?`
pub fn basic_test() -> Result<i32, i32> {
    let err: i32 = 1;
    // To avoid warnings during rustfix
        return Err(err);
    }
    Ok(0)
}
}

// Tests that `.into()` is added when appropriate
pub fn into_test() -> Result<i32, i32> {
    let err: u8 = 1;
    // To avoid warnings during rustfix
    if true {
        return Err(err.into());
    Ok(0)
}


// Tests that tries in general don't trigger the error
pub fn negative_test() -> Result<i32, i32> {
    Ok(nested_error()? + 1)


// Tests that `.into()` isn't added when the error type
// matches the surrounding closure's return type, even
// when it doesn't match the surrounding function's.
pub fn closure_matches_test() -> Result<i32, i32> {
    let res: Result<i32, i8> = Some(1)
        .into_iter()
        .map(|i| {
            let err: i8 = 1;
            // To avoid warnings during rustfix
                return Err(err);
            }
            Ok(i)
        })
        })
        .next()
        .unwrap();

    Ok(res?)
}

// Tests that `.into()` isn't added when the error type
// doesn't match the surrounding closure's return type.
pub fn closure_into_test() -> Result<i32, i32> {
    let res: Result<i32, i16> = Some(1)
        .into_iter()
        .map(|i| {
            let err: i8 = 1;
            // To avoid warnings during rustfix
            if true {
                return Err(err.into());
            Ok(i)
        })
        .next()
        .unwrap();
        .unwrap();

    Ok(res?)
}

fn nested_error() -> Result<i32, i32> {
    Ok(1)


// Bad suggestion when in macro (see #6242)
macro_rules! try_validation {
    ($e: expr) => {{
        match $e {
            Ok(_) => 0,
            Err(_) => return Err(1),
    }};
}

macro_rules! ret_one {
macro_rules! ret_one {
    () => {
        1
    };
}

macro_rules! try_validation_in_macro {
    ($e: expr) => {{
        match $e {
            Ok(_) => 0,
            Err(_) => return Err(ret_one!()),
    }};
}


fn calling_macro() -> Result<i32, i32> {
    // macro
    try_validation!(Ok::<_, i32>(5));
    // `Err` arg is another macro
    try_validation_in_macro!(Ok::<_, i32>(5));
    Ok(5)

fn main() {
    basic_test().unwrap();
    into_test().unwrap();
---
    // We don't want to lint in external macros
    try_err!();
}

macro_rules! bar {
    () => {
        String::from("aasdfasdfasdfa")
}

macro_rules! foo {
    () => {
    () => {
        bar!()
    };
}

pub fn macro_inside(fail: bool) -> Result<i32, String> {
    if fail {
        return Err(foo!());
    Ok(0)
}


pub fn poll_write(n: usize) -> Poll<io::Result<usize>> {
    if n == 0 {
        return Poll::Ready(Err(io::ErrorKind::WriteZero.into()))
    } else if n == 1 {
        return Poll::Ready(Err(io::Error::new(io::ErrorKind::InvalidInput, "error")))


    Poll::Ready(Ok(n))


pub fn poll_next(ready: bool) -> Poll<Option<io::Result<()>>> {
    if !ready {
        return Poll::Ready(Some(Err(io::ErrorKind::NotFound.into())))

    Poll::Ready(None)
}



diff of fixed:

 // run-rustfix
 // aux-build:macro_rules.rs
 
 #![deny(clippy::try_err)]
 #![allow(clippy::unnecessary_wraps, clippy::needless_question_mark)]
 #[macro_use]
 extern crate macro_rules;
 
 use std::io;
 use std::io;
 use std::task::Poll;
 
 // Tests that a simple case works
 // Should flag `Err(err)?`
 pub fn basic_test() -> Result<i32, i32> {
     let err: i32 = 1;
     // To avoid warnings during rustfix
-        return Err(err);
+        Err(err)?;
     }
     Ok(0)
     Ok(0)
 }
 
 // Tests that `.into()` is added when appropriate
 pub fn into_test() -> Result<i32, i32> {
     let err: u8 = 1;
     // To avoid warnings during rustfix
     if true {
-        return Err(err.into());
+        Err(err)?;
     Ok(0)
 }
 
 
 // Tests that tries in general don't trigger the error
 pub fn negative_test() -> Result<i32, i32> {
     Ok(nested_error()? + 1)
 
 
 // Tests that `.into()` isn't added when the error type
 // matches the surrounding closure's return type, even
 // when it doesn't match the surrounding function's.
 pub fn closure_matches_test() -> Result<i32, i32> {
     let res: Result<i32, i8> = Some(1)
         .into_iter()
         .map(|i| {
             let err: i8 = 1;
             // To avoid warnings during rustfix
-                return Err(err);
+                Err(err)?;
             }
             Ok(i)
             Ok(i)
         })
         .next()
         .unwrap();
 
     Ok(res?)
 }
 
 // Tests that `.into()` isn't added when the error type
 // doesn't match the surrounding closure's return type.
 pub fn closure_into_test() -> Result<i32, i32> {
     let res: Result<i32, i16> = Some(1)
         .into_iter()
         .map(|i| {
             let err: i8 = 1;
             // To avoid warnings during rustfix
             if true {
-                return Err(err.into());
+                Err(err)?;
             Ok(i)
         })
         .next()
         .unwrap();
         .unwrap();
 
     Ok(res?)
 }
 
 fn nested_error() -> Result<i32, i32> {
     Ok(1)
 
 
 // Bad suggestion when in macro (see #6242)
 macro_rules! try_validation {
     ($e: expr) => {{
         match $e {
             Ok(_) => 0,
-            Err(_) => return Err(1),
+            Err(_) => Err(1)?,
     }};
 }
 
 macro_rules! ret_one {
 macro_rules! ret_one {
     () => {
         1
     };
 }
 
 macro_rules! try_validation_in_macro {
     ($e: expr) => {{
         match $e {
             Ok(_) => 0,
-            Err(_) => return Err(ret_one!()),
+            Err(_) => Err(ret_one!())?,
     }};
 }
 
 
 fn calling_macro() -> Result<i32, i32> {
     // macro
     try_validation!(Ok::<_, i32>(5));
     // `Err` arg is another macro
     try_validation_in_macro!(Ok::<_, i32>(5));
     Ok(5)
 
 fn main() {
     basic_test().unwrap();
     into_test().unwrap();
---
     // We don't want to lint in external macros
     try_err!();
 }
 
 macro_rules! bar {
     () => {
         String::from("aasdfasdfasdfa")
 }
 
 macro_rules! foo {
     () => {
     () => {
         bar!()
     };
 }
 
 pub fn macro_inside(fail: bool) -> Result<i32, String> {
     if fail {
-        return Err(foo!());
+        Err(foo!())?;
     Ok(0)
 }
 
 
 pub fn poll_write(n: usize) -> Poll<io::Result<usize>> {
     if n == 0 {
-        return Poll::Ready(Err(io::ErrorKind::WriteZero.into()))
+        Err(io::ErrorKind::WriteZero)?
     } else if n == 1 {
-        return Poll::Ready(Err(io::Error::new(io::ErrorKind::InvalidInput, "error")))
+        Err(io::Error::new(io::ErrorKind::InvalidInput, "error"))?
 
 
     Poll::Ready(Ok(n))
 
 
 pub fn poll_next(ready: bool) -> Poll<Option<io::Result<()>>> {
     if !ready {
-        return Poll::Ready(Some(Err(io::ErrorKind::NotFound.into())))
+        Err(io::ErrorKind::NotFound)?
 
     Poll::Ready(None)
 }
 
 

The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-1989c91f44b16c80/out/test_build_base/try_err.fixed
To update references, run this command from build directory:
tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-1989c91f44b16c80/out/test_build_base' 'try_err.rs'
error: 2 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/try_err.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-1989c91f44b16c80/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-1989c91f44b16c80/out/test_build_base/try_err.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-3e5755171965ca17.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-45a3ec2898545ae5.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-efc540c81be69f24.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-1bcf52e6f16bdb4f.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-bde896d3fed1b934.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-1989c91f44b16c80/out/test_build_base/try_err.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------

normalized stderr:
error: written amount is not handled. Use `Write::write_all` instead
  --> $DIR/unused_io_amount.rs:14:5
   |
LL |     s.write(b"test").unwrap();
   |
   |
   = note: `-D clippy::unused-io-amount` implied by `-D warnings`

error: read amount is not handled. Use `Read::read_exact` instead
  --> $DIR/unused_io_amount.rs:16:5
   |
LL |     s.read(&mut buf).unwrap();

error: aborting due to 2 previous errors




expected stderr:
error: written amount is not handled. Use `Write::write_all` instead
  --> $DIR/unused_io_amount.rs:7:5
   |
LL |     s.write(b"test")?;
   |
   |
   = note: `-D clippy::unused-io-amount` implied by `-D warnings`

error: read amount is not handled. Use `Read::read_exact` instead
  --> $DIR/unused_io_amount.rs:9:5
   |
LL |     s.read(&mut buf)?;


error: written amount is not handled. Use `Write::write_all` instead
  --> $DIR/unused_io_amount.rs:14:5
   |
LL |     s.write(b"test").unwrap();


error: read amount is not handled. Use `Read::read_exact` instead
  --> $DIR/unused_io_amount.rs:16:5
   |
LL |     s.read(&mut buf).unwrap();

error: read amount is not handled
  --> $DIR/unused_io_amount.rs:20:5
   |
   |
LL |     s.read_vectored(&mut [io::IoSliceMut::new(&mut [])])?;

error: written amount is not handled
  --> $DIR/unused_io_amount.rs:21:5
   |
   |
LL |     s.write_vectored(&[io::IoSlice::new(&[])])?;

error: aborting due to 6 previous errors




diff of stderr:

 error: written amount is not handled. Use `Write::write_all` instead
-  --> $DIR/unused_io_amount.rs:7:5
-   |
-LL |     s.write(b"test")?;
-   |     ^^^^^^^^^^^^^^^^^
-   |
-   = note: `-D clippy::unused-io-amount` implied by `-D warnings`
-
-error: read amount is not handled. Use `Read::read_exact` instead
-  --> $DIR/unused_io_amount.rs:9:5
-   |
-LL |     s.read(&mut buf)?;
-   |     ^^^^^^^^^^^^^^^^^
-
-error: written amount is not handled. Use `Write::write_all` instead
   --> $DIR/unused_io_amount.rs:14:5
    |
 LL |     s.write(b"test").unwrap();
+   |
+   |
+   = note: `-D clippy::unused-io-amount` implied by `-D warnings`
 
 error: read amount is not handled. Use `Read::read_exact` instead
   --> $DIR/unused_io_amount.rs:16:5
    |
 LL |     s.read(&mut buf).unwrap();
 
-error: read amount is not handled
-  --> $DIR/unused_io_amount.rs:20:5
-   |
-   |
-LL |     s.read_vectored(&mut [io::IoSliceMut::new(&mut [])])?;
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-error: written amount is not handled
-  --> $DIR/unused_io_amount.rs:21:5
-   |
-   |
-LL |     s.write_vectored(&[io::IoSlice::new(&[])])?;
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-error: aborting due to 6 previous errors
+error: aborting due to 2 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-1989c91f44b16c80/out/test_build_base/unused_io_amount.stderr
To update references, run this command from build directory:
tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-1989c91f44b16c80/out/test_build_base' 'unused_io_amount.rs'
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/unused_io_amount.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-1989c91f44b16c80/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-1989c91f44b16c80/out/test_build_base/unused_io_amount.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-3e5755171965ca17.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-45a3ec2898545ae5.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-efc540c81be69f24.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-1bcf52e6f16bdb4f.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-bde896d3fed1b934.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-1989c91f44b16c80/out/test_build_base/unused_io_amount.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"written amount is not handled. Use `Write::write_all` instead","code":{"code":"clippy::unused_io_amount","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_io_amount.rs","byte_start":284,"byte_end":309,"line_start":14,"line_end":14,"column_start":5,"column_end":30,"is_primary":true,"text":[{"text":"    s.write(b\"test\").unwrap();","highlight_start":5,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::unused-io-amount` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: written amount is not handled. Use `Write::write_all` instead\n  --> tests/ui/unused_io_amount.rs:14:5\n   |\nLL |     s.write(b\"test\").unwrap();\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::unused-io-amount` implied by `-D warnings`\n\n"}
{"message":"read amount is not handled. Use `Read::read_exact` instead","code":{"code":"clippy::unused_io_amount","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_io_amount.rs","byte_start":343,"byte_end":368,"line_start":16,"line_end":16,"column_start":5,"column_end":30,"is_primary":true,"text":[{"text":"    s.read(&mut buf).unwrap();","highlight_start":5,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: read amount is not handled. Use `Read::read_exact` instead\n  --> tests/ui/unused_io_amount.rs:16:5\n   |\nLL |     s.read(&mut buf).unwrap();\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.5.0/src/lib.rs:105:22

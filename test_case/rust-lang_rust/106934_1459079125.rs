plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:46007752205b5430f5cabe1357251ea7621a9e98)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---

15    |
16    = note: this error originates in the macro `bug` (in Nightly builds, run with -Z macro-backtrace for more info)
17 
- error: unexpected expression: `{ let res = ::alloc::fmt::format(format_args!("{0}", "u8")); res }.as_str()`
+ error: unexpected expression: `{ let res = ::alloc::fmt::format(format_args!("{0}", "u8"); res }.as_str()`
19   --> $DIR/key-value-expansion.rs:48:23
20    |
21 LL |         doc_comment! {format!("{coor}", coor = stringify!($t1)).as_str()}

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/key-value-expansion/key-value-expansion.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args attributes/key-value-expansion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/attributes/key-value-expansion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/key-value-expansion" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/key-value-expansion/auxiliary"
stdout: none
--- stderr -------------------------------
error: unexpected expression: `(7u32)`
  --> fake-test-src-base/attributes/key-value-expansion.rs:21:6
   |
LL | bug!((column!())); //~ ERROR unexpected expression: `(7u32)`


error: unexpected expression: `"bug" + "found"`
  --> fake-test-src-base/attributes/key-value-expansion.rs:27:14
   |
LL |         bug!("bug" + stringify!(found)); //~ ERROR unexpected expression: `"bug" + "found"`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
...
...
LL | bug!();
   |
   = note: this error originates in the macro `bug` (in Nightly builds, run with -Z macro-backtrace for more info)


error: unexpected expression: `{ let res = ::alloc::fmt::format(format_args!("{0}", "u8"); res }.as_str()`
  --> fake-test-src-base/attributes/key-value-expansion.rs:48:23
   |
LL |         doc_comment! {format!("{coor}", coor = stringify!($t1)).as_str()}
...
...
LL | some_macro!(u8);
   |
   = note: this error originates in the macro `some_macro` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 3 previous errors
---
diff of stdout:

26 
27                 {
28                     ::std::rt::panic_fmt(format_args!("Assertion failed: elem as usize\nWith captures:\n  elem = {0:?}\n",
-                             __capture0))
+                             __capture0)
31             }
32     };


42                 (&::core::asserting::Wrapper(__local_bind0)).try_capture(&mut __capture0);
43                 {
44                     ::std::rt::panic_fmt(format_args!("Assertion failed: &elem\nWith captures:\n  elem = {0:?}\n",
-                             __capture0))
+                             __capture0)
47             }
48     };


58                 (&::core::asserting::Wrapper(__local_bind0)).try_capture(&mut __capture0);
59                 {
60                     ::std::rt::panic_fmt(format_args!("Assertion failed: elem == 1\nWith captures:\n  elem = {0:?}\n",
-                             __capture0))
+                             __capture0)
63             }
64     };


71                 (&::core::asserting::Wrapper(__local_bind0)).try_capture(&mut __capture0);
72                 {
73                     ::std::rt::panic_fmt(format_args!("Assertion failed: elem >= 1\nWith captures:\n  elem = {0:?}\n",
-                             __capture0))
+                             __capture0)
76             }
77     };


84                 (&::core::asserting::Wrapper(__local_bind0)).try_capture(&mut __capture0);
85                 {
86                     ::std::rt::panic_fmt(format_args!("Assertion failed: elem > 0\nWith captures:\n  elem = {0:?}\n",
-                             __capture0))
+                             __capture0)
89             }
90     };


97                 (&::core::asserting::Wrapper(__local_bind0)).try_capture(&mut __capture0);
98                 {
99                     ::std::rt::panic_fmt(format_args!("Assertion failed: elem < 3\nWith captures:\n  elem = {0:?}\n",
-                             __capture0))
+                             __capture0)
102             }
103     };


110                 (&::core::asserting::Wrapper(__local_bind0)).try_capture(&mut __capture0);
111                 {
112                     ::std::rt::panic_fmt(format_args!("Assertion failed: elem <= 3\nWith captures:\n  elem = {0:?}\n",
-                             __capture0))
+                             __capture0)
115             }
116     };


123                 (&::core::asserting::Wrapper(__local_bind0)).try_capture(&mut __capture0);
124                 {
125                     ::std::rt::panic_fmt(format_args!("Assertion failed: elem != 3\nWith captures:\n  elem = {0:?}\n",
-                             __capture0))
+                             __capture0)
128             }
129     };


139                 (&::core::asserting::Wrapper(__local_bind0)).try_capture(&mut __capture0);
140                 {
141                     ::std::rt::panic_fmt(format_args!("Assertion failed: *elem\nWith captures:\n  elem = {0:?}\n",
-                             __capture0))
+                             __capture0)
144             }
145     };



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/rfc-2011-nicer-assert-messages/non-consuming-methods-have-optimized-codegen/non-consuming-methods-have-optimized-codegen.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/rfc-2011-nicer-assert-messages/non-consuming-methods-have-optimized-codegen.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/macros/rfc-2011-nicer-assert-messages/non-consuming-methods-have-optimized-codegen.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/rfc-2011-nicer-assert-messages/non-consuming-methods-have-optimized-codegen" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/rfc-2011-nicer-assert-messages/non-consuming-methods-have-optimized-codegen/auxiliary" "-Z" "unpretty=expanded"
#![feature(prelude_import)]
#![no_std]
// check-pass
// check-pass
// compile-flags: -Z unpretty=expanded
#![feature(core_intrinsics, generic_assert, generic_assert_internals)]
#[prelude_import]
#[prelude_import]
use ::std::prelude::rust_2015::*;
extern crate std;


fn arbitrary_consuming_method_for_demonstration_purposes() {
    let elem = 1i32;
        #[allow(unused_imports)]
        #[allow(unused_imports)]
        use ::core::asserting::{TryCaptureGeneric, TryCapturePrintable};
        let mut __capture0 = ::core::asserting::Capture::new();
        let __local_bind0 = &elem;
        if ::core::intrinsics::unlikely(!(*{
                                    (&::core::asserting::Wrapper(__local_bind0)).try_capture(&mut __capture0);
                                } as usize)) {





                {
                    ::std::rt::panic_fmt(format_args!("Assertion failed: elem as usize\nWith captures:\n  elem = {0:?}\n",
                            __capture0)
            }
    };
}
fn addr_of() {
fn addr_of() {
    let elem = 1i32;
    {
        #[allow(unused_imports)]
        use ::core::asserting::{TryCaptureGeneric, TryCapturePrintable};
        let mut __capture0 = ::core::asserting::Capture::new();
        let __local_bind0 = &elem;
        if ::core::intrinsics::unlikely(!&*__local_bind0) {
                (&::core::asserting::Wrapper(__local_bind0)).try_capture(&mut __capture0);
                {
                    ::std::rt::panic_fmt(format_args!("Assertion failed: &elem\nWith captures:\n  elem = {0:?}\n",
                            __capture0)
            }
    };
}
fn binary() {
fn binary() {
    let elem = 1i32;
    {
        #[allow(unused_imports)]
        use ::core::asserting::{TryCaptureGeneric, TryCapturePrintable};
        let mut __capture0 = ::core::asserting::Capture::new();
        let __local_bind0 = &elem;
        if ::core::intrinsics::unlikely(!(*__local_bind0 == 1)) {
                (&::core::asserting::Wrapper(__local_bind0)).try_capture(&mut __capture0);
                {
                    ::std::rt::panic_fmt(format_args!("Assertion failed: elem == 1\nWith captures:\n  elem = {0:?}\n",
                            __capture0)
            }
    };
    {
        #[allow(unused_imports)]
        #[allow(unused_imports)]
        use ::core::asserting::{TryCaptureGeneric, TryCapturePrintable};
        let mut __capture0 = ::core::asserting::Capture::new();
        let __local_bind0 = &elem;
        if ::core::intrinsics::unlikely(!(*__local_bind0 >= 1)) {
                (&::core::asserting::Wrapper(__local_bind0)).try_capture(&mut __capture0);
                {
                    ::std::rt::panic_fmt(format_args!("Assertion failed: elem >= 1\nWith captures:\n  elem = {0:?}\n",
                            __capture0)
            }
    };
    {
        #[allow(unused_imports)]
        #[allow(unused_imports)]
        use ::core::asserting::{TryCaptureGeneric, TryCapturePrintable};
        let mut __capture0 = ::core::asserting::Capture::new();
        let __local_bind0 = &elem;
        if ::core::intrinsics::unlikely(!(*__local_bind0 > 0)) {
                (&::core::asserting::Wrapper(__local_bind0)).try_capture(&mut __capture0);
                {
                    ::std::rt::panic_fmt(format_args!("Assertion failed: elem > 0\nWith captures:\n  elem = {0:?}\n",
                            __capture0)
            }
    };
    {
        #[allow(unused_imports)]
        #[allow(unused_imports)]
        use ::core::asserting::{TryCaptureGeneric, TryCapturePrintable};
        let mut __capture0 = ::core::asserting::Capture::new();
        let __local_bind0 = &elem;
        if ::core::intrinsics::unlikely(!(*__local_bind0 < 3)) {
                (&::core::asserting::Wrapper(__local_bind0)).try_capture(&mut __capture0);
                {
                    ::std::rt::panic_fmt(format_args!("Assertion failed: elem < 3\nWith captures:\n  elem = {0:?}\n",
                            __capture0)
            }
    };
    {
        #[allow(unused_imports)]
        #[allow(unused_imports)]
        use ::core::asserting::{TryCaptureGeneric, TryCapturePrintable};
        let mut __capture0 = ::core::asserting::Capture::new();
        let __local_bind0 = &elem;
        if ::core::intrinsics::unlikely(!(*__local_bind0 <= 3)) {
                (&::core::asserting::Wrapper(__local_bind0)).try_capture(&mut __capture0);
                {
                    ::std::rt::panic_fmt(format_args!("Assertion failed: elem <= 3\nWith captures:\n  elem = {0:?}\n",
                            __capture0)
            }
    };
    {
        #[allow(unused_imports)]
        #[allow(unused_imports)]
        use ::core::asserting::{TryCaptureGeneric, TryCapturePrintable};
        let mut __capture0 = ::core::asserting::Capture::new();
        let __local_bind0 = &elem;
        if ::core::intrinsics::unlikely(!(*__local_bind0 != 3)) {
                (&::core::asserting::Wrapper(__local_bind0)).try_capture(&mut __capture0);
                {
                    ::std::rt::panic_fmt(format_args!("Assertion failed: elem != 3\nWith captures:\n  elem = {0:?}\n",
                            __capture0)
            }
    };
}
fn unary() {
fn unary() {
    let elem = &1i32;
    {
        #[allow(unused_imports)]
        use ::core::asserting::{TryCaptureGeneric, TryCapturePrintable};
        let mut __capture0 = ::core::asserting::Capture::new();
        let __local_bind0 = &elem;
        if ::core::intrinsics::unlikely(!**__local_bind0) {
                (&::core::asserting::Wrapper(__local_bind0)).try_capture(&mut __capture0);
                {
                    ::std::rt::panic_fmt(format_args!("Assertion failed: *elem\nWith captures:\n  elem = {0:?}\n",
                            __capture0)
            }
    };
}
fn main() {}


[01:25:08]      Running build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/system-05e4e3e335aa8f84
[01:25:08] 
[01:25:08] running 10 tests
[01:25:08] test format_lines_errors_are_reported ... ok
[01:25:08] test coverage_tests ... ok
[01:25:08] test checkstyle_test ... ok
[01:25:08] test rustfmt_diff_make_diff_tests ... ok
[01:25:08] test rustfmt_diff_no_diff_test ... ok
[01:25:08] test stdin_formatting_smoke_test ... ok
[01:25:08] test string_eq_ignore_newline_repr_test ... ok
[01:25:09] test system_tests ... FAILED
[01:25:09] test idempotence_tests ... FAILED
[01:25:09] test self_tests ... FAILED
[01:25:09] 
[01:25:09] failures:
[01:25:09] 
[01:25:09] ---- system_tests stdout ----
[01:25:09] 	
[01:25:09] Mismatch at tests/source/extern.rs:1:
[01:25:09]  // rustfmt-normalize_comments: true⏎
[01:25:09]  ⏎
[01:25:09] +extern crate foo⏎
[01:25:09]  extern crate foo as bar;⏎
[01:25:09] -extern crate foo;⏎
[01:25:09]  ⏎
[01:25:09] -extern crate chrono;⏎
[01:25:09] -extern crate dotenv;⏎
[01:25:09] -extern crate futures;⏎
[01:25:09] +extern crate chrono⏎
[01:25:09] +extern crate dotenv  // ;⏎
[01:25:09] +extern crate futures // ;;⏎
[01:25:09]  ⏎
[01:25:09] -extern crate bar;⏎
[01:25:09] -extern crate foo;⏎
[01:25:09] +extern crate bar⏎
[01:25:09] +extern crate foo // ;;⏎
[01:25:09]  ⏎
[01:25:09]  extern "C" {⏎
[01:25:09]      fn c_func(x: *mut *mut libc::c_void);⏎
[01:25:09] 
[01:25:09] Mismatch at tests/source/multiple.rs:6:
[01:25:09]  ⏎
[01:25:09]  ⏎
[01:25:09]  #[attr1]⏎
[01:25:09] -extern crate foo;⏎
[01:25:09] +extern crate foo // ;⏎
[01:25:09]  #[attr2]⏎
[01:25:09]  #[attr3]⏎
[01:25:09] -extern crate foo;⏎
[01:25:09] +extern crate foo // ;⏎
[01:25:09]  #[attr1]⏎
[01:25:09] -extern crate foo;⏎
[01:25:09] +extern crate foo // ;⏎
[01:25:09]  #[attr2]⏎
[01:25:09]  #[attr3]⏎
[01:25:09]  extern crate foo;⏎
[01:25:09] 
[01:25:09] Mismatch at tests/source/multiple.rs:55:
[01:25:09]      extern crate foo;⏎
[01:25:09]      #[attr2]⏎
[01:25:09]      #[attr3]⏎
[01:25:09] -    extern crate foo;⏎
[01:25:09] -}⏎
[01:25:09] +    extern crate foo;}⏎
[01:25:09]  ⏎
[01:25:09]  #[rustfmt_skip]⏎
[01:25:09]  fn qux(a: dadsfa,   // Comment 1⏎
[01:25:09] Ran 289 system tests.
[01:25:09] thread 'system_tests' panicked at 'assertion failed: `(left == right)`
[01:25:09]   left: `2`,
[01:25:09]  right: `0`: 2 system tests failed', /checkout/src/tools/rustfmt/tests/system.rs:52:4
[01:25:09] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:25:09] 
[01:25:09] ---- idempotence_tests stdout ----
[01:25:09] 	
[01:25:09] Mismatch at tests/target/extern.rs:1:
[01:25:09]  // rustfmt-normalize_comments: true⏎
[01:25:09]  ⏎
[01:25:09] -extern crate foo as bar;⏎
[01:25:09] -extern crate foo;⏎
[01:25:09] +extern crate foo⏎
[01:25:09] +extern crate foo as bar // ;;⏎
[01:25:09]  ⏎
[01:25:09] -extern crate chrono;⏎
[01:25:09] -extern crate dotenv;⏎
[01:25:09] +extern crate chrono // ;⏎
[01:25:09] +extern crate dotenv // ;⏎
[01:25:09]  extern crate futures;⏎
[01:25:09]  ⏎
[01:25:09] -extern crate bar;⏎
[01:25:09] +extern crate bar // ;⏎
[01:25:09]  extern crate foo;⏎
[01:25:09]  ⏎
[01:25:09]  extern "C" {⏎
[01:25:09] 
[01:25:09] Mismatch at tests/target/attrib-extern-crate.rs:1:
[01:25:09]  // Attributes on extern crate.⏎
[01:25:09]  ⏎
[01:25:09]  #[Attr1]⏎
[01:25:09] -extern crate Bar;⏎
[01:25:09] +extern crate Bar ;⏎
[01:25:09]  #[Attr2]⏎
[01:25:09]  #[Attr2]⏎
[01:25:09] -extern crate Baz;⏎
[01:25:09] +extern crate Baz ;⏎
[01:25:09]  extern crate Foo;⏎
[01:25:09]  ⏎
[01:25:09]  fn foo() {⏎
[01:25:09] 
[01:25:09] Mismatch at tests/target/attrib-extern-crate.rs:13:
[01:25:09]      #[Attr2]⏎
[01:25:09]      #[Attr2]⏎
[01:25:09]      extern crate Baz;⏎
[01:25:09] -    extern crate Foo;⏎
[01:25:09] -}⏎
[01:25:09] +    extern crate Foo;}⏎
[01:25:09] 
[01:25:09] Mismatch at tests/target/multiple.rs:6:
[01:25:09]  ⏎
[01:25:09]  ⏎
[01:25:09]  #[attr1]⏎
[01:25:09] -extern crate foo;⏎
[01:25:09] +extern crate foo // ;⏎
[01:25:09]  #[attr2]⏎
[01:25:09]  #[attr3]⏎
[01:25:09] -extern crate foo;⏎
[01:25:09] +extern crate foo // ;⏎
[01:25:09]  #[attr1]⏎
[01:25:09] -extern crate foo;⏎
[01:25:09] +extern crate foo // ;⏎
[01:25:09]  #[attr2]⏎
[01:25:09]  #[attr3]⏎
[01:25:09]  extern crate foo;⏎
[01:25:09] 
[01:25:09] Mismatch at tests/target/multiple.rs:55:
[01:25:09]      extern crate foo;⏎
[01:25:09]      #[attr2]⏎
[01:25:09]      #[attr3]⏎
[01:25:09] -    extern crate foo;⏎
[01:25:09] -}⏎
[01:25:09] +    extern crate foo;}⏎
[01:25:09]  ⏎
[01:25:09]  #[rustfmt_skip]⏎
[01:25:09]  fn qux(a: dadsfa,   // Comment 1⏎
[01:25:09] Ran 339 idempotent tests.
[01:25:09] thread 'idempotence_tests' panicked at 'assertion failed: `(left == right)`
[01:25:09]   left: `3`,
[01:25:09]  right: `0`: 3 idempotent tests failed', /checkout/src/tools/rustfmt/tests/system.rs:113:4
[01:25:09] 
[01:25:09] ---- self_tests stdout ----
[01:25:09] 	
[01:25:09] Mismatch at src/bin/cargo-fmt.rs:13:
[01:25:09]  #![cfg(not(test))]⏎
[01:25:09]  #![deny(warnings)]⏎
[01:25:09]  ⏎
[01:25:09] -extern crate getopts;⏎
[01:25:09] +extern crate getopts ;⏎
[01:25:09]  extern crate serde_json as json;⏎
[01:25:09]  ⏎
[01:25:09]  use std::env;⏎
[01:25:09] 
[01:25:09] Mismatch at src/bin/rustfmt-format-diff.rs:14:
[01:25:09]  ⏎
[01:25:09]  #![deny(warnings)]⏎
[01:25:09]  ⏎
[01:25:09] -extern crate env_logger;⏎
[01:25:09] -extern crate getopts;⏎
[01:25:09] +extern crate env_logger ;⏎
[01:25:09] +extern crate getopts    ;⏎
[01:25:09]  #[macro_use]⏎
[01:25:09] -extern crate log;⏎
[01:25:09] -extern crate regex;⏎
[01:25:09] +extern crate log ;⏎
[01:25:09] +extern crate regex      ;⏎
[01:25:09]  #[macro_use]⏎
[01:25:09] -extern crate serde_derive;⏎
[01:25:09] +extern crate serde_derive ;⏎
[01:25:09]  extern crate serde_json as json;⏎
[01:25:09]  ⏎
[01:25:09]  use std::{env, fmt, process};⏎
[01:25:09] 
[01:25:09] Mismatch at src/bin/rustfmt.rs:11:
[01:25:09]  #![cfg(not(test))]⏎
[01:25:09]  ⏎
[01:25:09]  ⏎
[01:25:09] -extern crate env_logger;⏎
[01:25:09] -extern crate getopts;⏎
[01:25:09] +extern crate env_logger ;⏎
[01:25:09] +extern crate getopts    ;⏎
[01:25:09]  extern crate rustfmt_nightly as rustfmt;⏎
[01:25:09]  ⏎
[01:25:09]  use std::{env, error};⏎
[01:25:09] 
[01:25:09] Mismatch at tests/system.rs:9:
[01:25:09]  // except according to those terms.⏎
[01:25:09]  ⏎
[01:25:09]  #[macro_use]⏎
[01:25:09] -extern crate log;⏎
[01:25:09] -extern crate regex;⏎
[01:25:09] -extern crate rustfmt_nightly as rustfmt;⏎
[01:25:09] +extern crate log ;⏎
[01:25:09] +extern crate regex                      ;⏎
[01:25:09] +extern crate rustfmt_nightly as rustfmt ;⏎
[01:25:09]  extern crate term;⏎
[01:25:09]  ⏎
[01:25:09]  use std::collections::HashMap;⏎
[01:25:09] 
[01:25:09] Mismatch at src/lib.rs:10:
[01:25:09]  ⏎
[01:25:09]  #![feature(rustc_private)]⏎
[01:25:09]  ⏎
[01:25:09] -extern crate diff;⏎
[01:25:09] +extern crate diff ;⏎
[01:25:09]  #[macro_use]⏎
[01:25:09] -extern crate log;⏎
[01:25:09] -extern crate regex;⏎
[01:25:09] -extern crate rustc_errors as errors;⏎
[01:25:09] -extern crate serde;⏎
[01:25:09] +extern crate log ;⏎
[01:25:09] +extern crate regex ;⏎
[01:25:09] +extern crate rustc_errors as errors ;⏎
[01:25:09] +extern crate serde ;⏎
[01:25:09]  #[macro_use]⏎
[01:25:09] -extern crate serde_derive;⏎
[01:25:09] -extern crate serde_json;⏎
[01:25:09] -extern crate strings;⏎
[01:25:09] -extern crate syntax;⏎
[01:25:09] -extern crate term;⏎
[01:25:09] +extern crate serde_derive ;⏎
[01:25:09] +extern crate serde_json ;⏎
[01:25:09] +extern crate strings ;⏎
[01:25:09] +extern crate syntax ;⏎
[01:25:09] +extern crate term ;⏎
[01:25:09]  extern crate unicode_segmentation;⏎
[01:25:09]  ⏎
[01:25:09]  use std::collections::HashMap;⏎
[01:25:09] Ran 6 self tests.
[01:25:09] thread 'self_tests' panicked at 'assertion failed: `(left == right)`
[01:25:09]   left: `5`,
[01:25:09]  right: `0`: 5 self tests failed', /checkout/src/tools/rustfmt/tests/system.rs:133:4
[01:25:09] 
[01:25:09] 
[01:25:09] failures:
[01:25:09]     idempotence_tests
[01:25:09]     self_tests
[01:25:09]     system_tests
[01:25:09] 
[01:25:09] test result: FAILED. 7 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
[01:25:09] 
[01:25:09] error: test failed, to rerun pass '--test system'
[01:25:09] 
[01:25:09] 
[01:25:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustfmt/Cargo.toml"
[01:25:09] expected success, got: exit code: 101
[01:25:09] 
[01:25:09] 
[01:25:09] You can disable the tool in `src/tools/toolstate.toml`
[01:25:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/cargo src/tools/rls src/tools/rustfmt src/tools/miri src/test/pretty src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/test/run-fail-fulldeps/pretty
[01:25:09] Build completed unsuccessfully in 0:34:18
[01:25:09] Makefile:54: recipe for target 'check-aux' failed
[01:25:09] make: *** [check-aux] Error 1

plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8f4b7f84864484a7bf31766abe9204da3cbe65b3)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3040325909b538d8ad81ad89a04b7a91b109c313)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
failures:

---- [ui] tests/rustdoc-ui/run-directory.rs#incorrect stdout ----

error in revision `incorrect`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/run-directory.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "incorrect" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/run-directory.incorrect" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/run-directory.incorrect/auxiliary" "--test" "--test-run-directory=/checkout/tests/rustdoc-ui/coverage"
running 1 test
test /checkout/tests/rustdoc-ui/run-directory.rs - foo (line 20) ... FAILED

failures:
failures:

---- /checkout/tests/rustdoc-ui/run-directory.rs - foo (line 20) stdout ----
Test executable failed (exit status: 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `"/checkout/tests/rustdoc-ui/run-directory.rs"`,
 right: `"tests/rustdoc-ui/run-directory.rs"`', /checkout/tests/rustdoc-ui/run-directory.rs:4:1
stack backtrace:
   0:     0x55c42908a945 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6a202ee890c7a979
   1:     0x55c4290a65e8 - core::fmt::write::hf82bb59cd0f235c9
   2:     0x55c429088ca1 - std::io::Write::write_fmt::hb00b2b69b4505999
   3:     0x55c42908a751 - std::sys_common::backtrace::print::h4b52bee7e0a16f8b
   4:     0x55c42908bf44 - std::panicking::default_hook::{{closure}}::hbe446d2536f68c7c
   5:     0x55c42908bc2a - std::panicking::default_hook::h2ea24407f8093ca4
   6:     0x55c42908c401 - std::panicking::rust_panic_with_hook::hfa435e338fca656c
   7:     0x55c42908c2e9 - std::panicking::begin_panic_handler::{{closure}}::h6c6cbb843e04d78e
   8:     0x55c42908ae16 - std::sys_common::backtrace::__rust_end_short_backtrace::h50d3769ff93a5026
   9:     0x55c42908bfc7 - rust_begin_unwind
  10:     0x55c429063c93 - core::panicking::panic_fmt::h02855701657f1a92
  11:     0x55c429063f0c - core::panicking::assert_failed_inner::h25115d4476675c8a
  12:     0x55c429066bea - core::panicking::assert_failed::h82899ab2f1e82f67
  13:     0x55c429069172 - rust_out::main::_doctest_main__checkout_tests_rustdoc_ui_run_directory_rs_20_0::haec5a6403a918eb8
  14:     0x55c429068fe6 - rust_out::main::h2c59cabdc70bcdf9
  15:     0x55c4290655d3 - core::ops::function::FnOnce::call_once::hfcec541b85f5a8c4
  16:     0x55c4290646c9 - std::sys_common::backtrace::__rust_begin_short_backtrace::h0b8b387afc11a6d8
  17:     0x55c429064e59 - std::rt::lang_start::{{closure}}::hdb7bf6aa9d46d515
  18:     0x55c42908669c - std::rt::lang_start_internal::hf2976b8d2a990aab
  19:     0x55c429064e37 - std::rt::lang_start::he7cdb2e4273993d3
  20:     0x55c429069195 - main
  21:     0x7f81d7e91d90 - <unknown>
  23:     0x55c4290641c5 - _start
  24:                0x0 - <unknown>


---


---- [ui] tests/rustdoc-ui/run-directory.rs#correct stdout ----

error in revision `correct`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/run-directory.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "correct" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/run-directory.correct" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/run-directory.correct/auxiliary" "--test" "--test-run-directory=/checkout/tests/rustdoc-ui"
running 1 test
test /checkout/tests/rustdoc-ui/run-directory.rs - foo (line 10) ... FAILED

failures:
failures:

---- /checkout/tests/rustdoc-ui/run-directory.rs - foo (line 10) stdout ----
Test executable failed (exit status: 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `"/checkout/tests/rustdoc-ui/run-directory.rs"`,
 right: `"tests/rustdoc-ui/run-directory.rs"`', /checkout/tests/rustdoc-ui/run-directory.rs:7:1
   0:     0x561b1b961de5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6a202ee890c7a979
   0:     0x561b1b961de5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6a202ee890c7a979
   1:     0x561b1b97db28 - core::fmt::write::hf82bb59cd0f235c9
   2:     0x561b1b960141 - std::io::Write::write_fmt::hb00b2b69b4505999
   3:     0x561b1b961bf1 - std::sys_common::backtrace::print::h4b52bee7e0a16f8b
   4:     0x561b1b9633e4 - std::panicking::default_hook::{{closure}}::hbe446d2536f68c7c
   5:     0x561b1b9630ca - std::panicking::default_hook::h2ea24407f8093ca4
   6:     0x561b1b9638a1 - std::panicking::rust_panic_with_hook::hfa435e338fca656c
   7:     0x561b1b963789 - std::panicking::begin_panic_handler::{{closure}}::h6c6cbb843e04d78e
   8:     0x561b1b9622b6 - std::sys_common::backtrace::__rust_end_short_backtrace::h50d3769ff93a5026
   9:     0x561b1b963467 - rust_begin_unwind
  10:     0x561b1b93ac93 - core::panicking::panic_fmt::h02855701657f1a92
  11:     0x561b1b93af0c - core::panicking::assert_failed_inner::h25115d4476675c8a
  12:     0x561b1b93dcaa - core::panicking::assert_failed::h82899ab2f1e82f67
  13:     0x561b1b940308 - rust_out::main::_doctest_main__checkout_tests_rustdoc_ui_run_directory_rs_10_0::hb87c919c1428f01f
  14:     0x561b1b940116 - rust_out::main::h2c59cabdc70bcdf9
  15:     0x561b1b93c5d3 - core::ops::function::FnOnce::call_once::hfcec541b85f5a8c4
  16:     0x561b1b93b6c9 - std::sys_common::backtrace::__rust_begin_short_backtrace::h0b8b387afc11a6d8
  17:     0x561b1b93be59 - std::rt::lang_start::{{closure}}::hdb7bf6aa9d46d515
  18:     0x561b1b95d8ec - std::rt::lang_start_internal::hf2976b8d2a990aab
  19:     0x561b1b93be37 - std::rt::lang_start::he7cdb2e4273993d3
  20:     0x561b1b940325 - main
  21:     0x7f7d13d18d90 - <unknown>
  22:     0x7f7d13d18e40 - __libc_start_main
  23:     0x561b1b93b1c5 - _start
  24:                0x0 - <unknown>


failures:
    /checkout/tests/rustdoc-ui/run-directory.rs - foo (line 10)

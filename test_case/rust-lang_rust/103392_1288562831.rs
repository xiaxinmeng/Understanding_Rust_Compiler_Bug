plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 7feb003882ecf7699e6705b537673e20985accff and 225063b46d6c62cd429cde8dbd4fb483ccf72615
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
Successfully built 21243eace474
Successfully tagged rust-ci:latest
Built container sha256:21243eace474d6d67b5e26092883df3b2b4b42e8a6686b62042ac2602f8c31b9
Uploading finished image to https://ci-caches.rust-lang.org/docker/268652770c9de9c4957e9a4232dca9618ebed062133dc91a39734cd3934b4ebd3c584468ed097890900591b1522076a5a39519d33aa6bda6e4ca5fe4abb76da3
upload failed: - to s3://rust-lang-ci-sccache2/docker/268652770c9de9c4957e9a4232dca9618ebed062133dc91a39734cd3934b4ebd3c584468ed097890900591b1522076a5a39519d33aa6bda6e4ca5fe4abb76da3 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-tools]
---
tests/pass/shims/env/home.rs ... ok
tests/pass/shims/env/args.rs ... ok
tests/pass/stacked-borrows/stacked-borrows.rs ... ok
tests/pass/shims/env/var-forward.rs ... ok
tests/pass/stacked-borrows/unknown-bottom-gc.rs ... ok
tests/pass/panic/catch_panic.rs ... ok
tests/pass/concurrency/sync.rs ... ok
tests/pass/weak_memory/weak.rs ... ok
tests/pass/shims/env/var.rs ... ok
---
  Downloaded serde_derive v1.0.145
  Downloaded byteorder v0.5.3
   Compiling proc-macro2 v1.0.44
   Compiling byteorder v1.4.3
   Compiling issue_1760 v0.1.0 (/checkout/src/tools/miri/test-cargo-miri/issue-1760)
   Compiling exported_symbol_dep v0.1.0 (/checkout/src/tools/miri/test-cargo-miri/exported-symbol-dep)
   Compiling serde_derive v1.0.145
   Compiling serde_derive v1.0.145
   Compiling issue_1691 v0.1.0 (/checkout/src/tools/miri/test-cargo-miri/issue-1691)
   Compiling issue_rust_86261 v0.1.0 (/checkout/src/tools/miri/test-cargo-miri/issue-rust-86261)
   Compiling cargo-miri-test v0.1.0 (/checkout/src/tools/miri/test-cargo-miri)
   Compiling exported_symbol v0.1.0 (/checkout/src/tools/miri/test-cargo-miri/exported-symbol)
   Compiling exported_symbol v0.1.0 (/checkout/src/tools/miri/test-cargo-miri/exported-symbol)
   Compiling issue_1705 v0.1.0 (/checkout/src/tools/miri/test-cargo-miri/issue-1705)
   Compiling issue_1567 v0.1.0 (/checkout/src/tools/miri/test-cargo-miri/issue-1567)
   Compiling cdylib v0.1.0 (/checkout/src/tools/miri/test-cargo-miri/cdylib)
    Finished test [unoptimized + debuginfo] target(s) in 12.89s
     Running unittests src/main.rs (obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/cargo_miri_test-f318a0d4bb012bd6)

running 2 tests
---
tests/pass/weak_memory/extra_cpp_unsafe.rs ... ok
tests/pass/stacked-borrows/stacked-borrows.rs ... ok
tests/pass/shims/env/args.rs ... ok
tests/pass/shims/env/var-forward.rs ... ok
tests/pass/stacked-borrows/unknown-bottom-gc.rs ... ok
tests/pass/shims/env/current_dir.rs ... ok
tests/pass/concurrency/tls_lib_drop.rs ... ok
tests/pass/panic/catch_panic.rs ... ok
tests/pass/weak_memory/weak.rs ... ok
---
+++ <stderr output>
+error: unsupported operation: can't call foreign function: GetFileType
+  --> RUSTLIB/std/src/sys/PLATFORM/io.rs:LL:CC
+   |
+LL |     if c::GetFileType(handle) != c::FILE_TYPE_PIPE {
+   |
+   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
+   = note: BACKTRACE:
+   = note: BACKTRACE:
+   = note: inside `std::sys::PLATFORM::io::msys_tty_on` at RUSTLIB/std/src/sys/PLATFORM/io.rs:LL:CC
+   = note: inside `std::sys::PLATFORM::io::handle_is_console` at RUSTLIB/std/src/sys/PLATFORM/io.rs:LL:CC
+   = note: inside `std::sys::PLATFORM::io::is_terminal::<std::io::Stdout>` at RUSTLIB/std/src/sys/PLATFORM/io.rs:LL:CC
+   = note: inside `<std::io::Stdout as std::io::IsTerminal>::is_terminal` at RUSTLIB/std/src/io/stdio.rs:LL:CC
+note: inside `main` at $DIR/io.rs:LL:CC
+  --> $DIR/io.rs:LL:CC
+   |
+LL |     std::io::stdout().is_terminal();
+
+note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace
+
+error: aborting due to previous error
---
full stderr:
error: unsupported operation: can't call foreign function: GetFileType
  --> /checkout/library/std/src/sys/windows/io.rs:124:8
   |
LL |     if c::GetFileType(handle) != c::FILE_TYPE_PIPE {
   |
   = help: this is likely not a bug in the program; it indicates that the program performed an operation that the interpreter does not support
   = note: BACKTRACE:
   = note: BACKTRACE:
   = note: inside `std::sys::windows::io::msys_tty_on` at /checkout/library/std/src/sys/windows/io.rs:124:8
   = note: inside `std::sys::windows::io::handle_is_console` at /checkout/library/std/src/sys/windows/io.rs:119:5
   = note: inside `std::sys::windows::io::is_terminal::<std::io::Stdout>` at /checkout/library/std/src/sys/windows/io.rs:87:14
   = note: inside `<std::io::Stdout as std::io::IsTerminal>::is_terminal` at /checkout/library/std/src/io/stdio.rs:1059:17
note: inside `main` at tests/pass/shims/io.rs:8:5
  --> tests/pass/shims/io.rs:8:5
   |
LL |     std::io::stdout().is_terminal();

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

error: aborting due to previous error

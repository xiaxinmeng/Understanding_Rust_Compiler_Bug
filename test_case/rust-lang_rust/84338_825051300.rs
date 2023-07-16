plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Finished release [optimized] target(s) in 10.57s
tidy check
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
tidy error: /checkout/src/etc/lldb_providers.py:613: line longer than 100 chars
tidy error: /checkout/src/etc/lldb_providers.py:614: line longer than 100 chars
tidy error: /checkout/src/etc/lldb_providers.py:636: line longer than 100 chars
tidy error: /checkout/src/etc/lldb_providers.py:648: line longer than 100 chars
tidy error: /checkout/src/etc/lldb_providers.py:649: line longer than 100 chars
tidy error: /checkout/src/etc/lldb_providers.py:651: line longer than 100 chars
tidy error: /checkout/src/etc/lldb_providers.py:652: line longer than 100 chars
tidy error: /checkout/src/etc/gdb_providers.py:159: line longer than 100 chars
tidy error: /checkout/src/etc/gdb_providers.py:161: line longer than 100 chars
* highest error code: E0781
Found 516 error codes
Found 0 error codes with no tests
Done!
Done!
* 326 features
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:18

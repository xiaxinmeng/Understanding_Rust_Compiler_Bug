plain

testing https://github.com/servo/servo
Initialized empty Git repository in /checkout/obj/build/ct/servo/.git/
fatal: Could not parse object 'caac107ae8145ef2fd20365e2b8fadaf09c2eb3b'.
error: RPC failed; curl 56 GnuTLS recv error (-54): Error in the pull function.
fatal: early EOF
fatal: index-pack failed
fatal: index-pack failed
thread 'main' panicked at 'assertion failed: status.success()', src/tools/cargotest/main.rs:123:13


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/cargotest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/cargo" "/checkout/obj/build/ct"
expected success, got: exit code: 101
expected success, got: exit code: 101


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/cargo src/tools/cargotest
Build completed unsuccessfully in 0:26:17
make: *** [check-aux] Error 1
Makefile:44: recipe for target 'check-aux' failed

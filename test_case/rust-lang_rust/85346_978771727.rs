plain
error: RPC failed; curl 56 GnuTLS recv error (-54): Error in the pull function.
fatal: The remote end hung up unexpectedly
fatal: early EOF
fatal: index-pack failed
thread 'main' panicked at 'assertion failed: status.success()', src/tools/cargotest/main.rs:125:13


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/cargotest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/cargo" "/checkout/obj/build/ct"
expected success, got: exit status: 101
expected success, got: exit status: 101


Build completed unsuccessfully in 0:26:36
Makefile:44: recipe for target 'check-aux' failed
make: *** [check-aux] Error 1

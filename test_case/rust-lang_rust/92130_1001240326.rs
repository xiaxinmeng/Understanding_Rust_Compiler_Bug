plain
fatal: Could not parse object 'caac107ae8145ef2fd20365e2b8fadaf09c2eb3b'.
From https://github.com/servo/servo
 * branch            master     -> FETCH_HEAD
fatal: Could not parse object 'caac107ae8145ef2fd20365e2b8fadaf09c2eb3b'.
error: RPC failed; curl 56 GnuTLS recv error (-9): A TLS packet with unexpected length was received.
fatal: early EOF
fatal: index-pack failed
fatal: index-pack failed
thread 'main' panicked at 'assertion failed: status.success()', src/tools/cargotest/main.rs:125:13


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/cargotest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/cargo" "/checkout/obj/build/ct"
expected success, got: exit status: 101
expected success, got: exit status: 101


Build completed unsuccessfully in 1:03:33
make: *** [check-aux] Error 1
Makefile:44: recipe for target 'check-aux' failed

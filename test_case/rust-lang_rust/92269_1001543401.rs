plain
fatal: Could not parse object '91493fe47175076f330ce5fc518f0196c0476f56'.
From https://github.com/diesel-rs/diesel
 * branch            master     -> FETCH_HEAD
fatal: Could not parse object '91493fe47175076f330ce5fc518f0196c0476f56'.
fatal: unable to access 'https://github.com/diesel-rs/diesel/': gnutls_handshake() failed: Error in the pull function.
thread 'main' panicked at 'assertion failed: status.success()', src/tools/cargotest/main.rs:125:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/cargotest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/cargo" "/checkout/obj/build/ct"
expected success, got: exit status: 101
expected success, got: exit status: 101


Build completed unsuccessfully in 0:25:27
make: *** [check-aux] Error 1
Makefile:44: recipe for target 'check-aux' failed

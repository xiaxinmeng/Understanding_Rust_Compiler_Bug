plain
fatal: Could not parse object '3de6c04269a7d315f7e9864b9013451cd9580a08'.
From https://github.com/BurntSushi/xsv
 * branch            master     -> FETCH_HEAD
fatal: Could not parse object '3de6c04269a7d315f7e9864b9013451cd9580a08'.
fatal: unable to access 'https://github.com/BurntSushi/xsv/': Failed to connect to github.com port 443: Connection timed out
thread 'main' panicked at 'assertion failed: status.success()', src/tools/cargotest/main.rs:125:13


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/cargotest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/cargo" "/checkout/obj/build/ct"
expected success, got: exit status: 101
expected success, got: exit status: 101


Build completed unsuccessfully in 0:31:19
make: *** [check-aux] Error 1
Makefile:44: recipe for target 'check-aux' failed

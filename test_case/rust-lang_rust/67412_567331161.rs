plain
2019-12-19T04:46:43.1092230Z 
2019-12-19T04:46:43.1092269Z 
2019-12-19T04:46:43.1092331Z failures:
2019-12-19T04:46:43.1092415Z 
2019-12-19T04:46:43.1092768Z ---- [run-make] run-make/removing-code-and-incremental-lto stdout ----
2019-12-19T04:46:43.1092920Z error: make failed
2019-12-19T04:46:43.1093004Z status: exit code: 2
2019-12-19T04:46:43.1093071Z command: "make" "make"
2019-12-19T04:46:43.1093153Z stdout:
2019-12-19T04:46:43.1093153Z stdout:
2019-12-19T04:46:43.1093396Z ------------------------------------------
2019-12-19T04:46:43.1093772Z mkdir -p /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/removing-code-and-incremental-lto/removing-code-and-incremental-lto
2019-12-19T04:46:43.1094169Z mkdir -p /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/removing-code-and-incremental-lto/removing-code-and-incremental-lto/incr
2019-12-19T04:46:43.1094619Z mkdir -p /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/removing-code-and-incremental-lto/removing-code-and-incremental-lto/rubble1
2019-12-19T04:46:43.1095063Z mkdir -p /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/removing-code-and-incremental-lto/removing-code-and-incremental-lto/rubble2
2019-12-19T04:46:43.1095776Z /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --crate-name cortex_m_rt cortex-m-rt.rs --crate-type lib --emit=metadata,link -C opt-level=s --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/removing-code-and-incremental-lto/removing-code-and-incremental-lto --target thumbv6m-none-eabi -C link-arg=-Tlink.x.in -L .
2019-12-19T04:46:43.1096819Z /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition=2018 --crate-name nrf52810_hal nrf52810-hal.rs --crate-type lib --emit=metadata,link -C opt-level=s -C metadata=aa86958b67bf89f5 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/removing-code-and-incremental-lto/removing-code-and-incremental-lto --target thumbv6m-none-eabi --extern cortex_m_rt=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/removing-code-and-incremental-lto/removing-code-and-incremental-lto/libcortex_m_rt.rmeta -C link-arg=-Tlink.x.in -L .
2019-12-19T04:46:43.1097675Z cp rubble.rs.v1 /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/removing-code-and-incremental-lto/removing-code-and-incremental-lto/rubble.rs
2019-12-19T04:46:43.1099506Z /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --crate-name rubble /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/removing-code-and-incremental-lto/removing-code-and-incremental-lto/rubble.rs --crate-type bin --emit=link -C opt-level=s --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/removing-code-and-incremental-lto/removing-code-and-incremental-lto/rubble1 --target thumbv6m-none-eabi -C incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/removing-code-and-incremental-lto/removing-code-and-incremental-lto/incr -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/removing-code-and-incremental-lto/removing-code-and-incremental-lto --extern nrf52810_hal=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/removing-code-and-incremental-lto/removing-code-and-incremental-lto/libnrf52810_hal.rlib -C link-arg=-Tlink.x.in -L . -C linker-flavor=ld.lld -C codegen-units=2
2019-12-19T04:46:43.1100186Z Makefile:69: recipe for target 'all' failed
2019-12-19T04:46:43.1100484Z ------------------------------------------
2019-12-19T04:46:43.1100573Z stderr:
2019-12-19T04:46:43.1100972Z ------------------------------------------
2019-12-19T04:46:43.1100972Z ------------------------------------------
2019-12-19T04:46:43.1101066Z warning: unused variable: `submilli_micros`
2019-12-19T04:46:43.1101492Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/removing-code-and-incremental-lto/removing-code-and-incremental-lto/rubble.rs:25:30
2019-12-19T04:46:43.1101623Z    |
2019-12-19T04:46:43.1101875Z 25 |                 let (millis, submilli_micros) = (self.0 / 1000, self.0 % 1000);
2019-12-19T04:46:43.1102202Z    |                              ^^^^^^^^^^^^^^^ help: consider prefixing with an underscore: `_submilli_micros`
2019-12-19T04:46:43.1102389Z    = note: `#[warn(unused_variables)]` on by default
2019-12-19T04:46:43.1102438Z 
2019-12-19T04:46:43.1102438Z 
2019-12-19T04:46:43.1102516Z error: linker `lld` not found
2019-12-19T04:46:43.1102672Z   = note: No such file or directory (os error 2)
2019-12-19T04:46:43.1102719Z 
2019-12-19T04:46:43.1102795Z error: aborting due to previous error
2019-12-19T04:46:43.1102839Z 
2019-12-19T04:46:43.1102839Z 
2019-12-19T04:46:43.1102913Z make: *** [all] Error 1
2019-12-19T04:46:43.1103225Z ------------------------------------------
2019-12-19T04:46:43.1103275Z 
2019-12-19T04:46:43.1103309Z 
2019-12-19T04:46:43.1103342Z 
2019-12-19T04:46:43.1103342Z 
2019-12-19T04:46:43.1103418Z failures:
2019-12-19T04:46:43.1103672Z     [run-make] run-make/removing-code-and-incremental-lto
2019-12-19T04:46:43.1104208Z test result: FAILED. 3 passed; 1 failed; 9 ignored; 0 measured; 0 filtered out
2019-12-19T04:46:43.1104291Z 
2019-12-19T04:46:43.1111462Z 
2019-12-19T04:46:43.1111561Z 
2019-12-19T04:46:43.1111561Z 
2019-12-19T04:46:43.1113773Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-make" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make" "--stage-id" "stage2-thumbv6m-none-eabi" "--mode" "run-make" "--target" "thumbv6m-none-eabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "arm-none-eabi-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/thumbv6m-none-eabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--llvm-version" "9.0.0-rust-1.42.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-19T04:46:43.1114486Z 
2019-12-19T04:46:43.1114523Z 
2019-12-19T04:46:43.1121990Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf src/test/run-make
2019-12-19T04:46:43.1122148Z Build completed unsuccessfully in 1:00:09
2019-12-19T04:46:43.1122148Z Build completed unsuccessfully in 1:00:09
2019-12-19T04:46:43.1176122Z == clock drift check ==
2019-12-19T04:46:43.1195207Z   local time: Thu Dec 19 04:46:43 UTC 2019
2019-12-19T04:46:43.6495719Z   network time: Thu, 19 Dec 2019 04:46:43 GMT
2019-12-19T04:46:43.6496120Z == end clock drift check ==
2019-12-19T04:46:45.3349515Z 
2019-12-19T04:46:45.3453661Z ##[error]Bash exited with code '1'.
2019-12-19T04:46:45.3519537Z ##[section]Starting: Checkout
2019-12-19T04:46:45.3521688Z ==============================================================================
2019-12-19T04:46:45.3521780Z Task         : Get sources
2019-12-19T04:46:45.3521878Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.

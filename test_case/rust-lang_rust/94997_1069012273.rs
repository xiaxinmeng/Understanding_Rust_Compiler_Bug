plain
.................................................................................................... 2600/12741
....................................................i............................................... 2700/12741
..............i.i..........................i..............i..............................i.......... 2800/12741
.................................................................................................... 2900/12741
..........i................................F..F..................................................... 3000/12741
...............................iiiii................................................................ 3200/12741
.................................................................................................... 3300/12741
.................................................................................................... 3400/12741
.................................................................................................... 3500/12741
---
....................................iii............................................................. 12700/12741
.........................................
failures:

---- [ui] ui/debuginfo/debuginfo-type-name-layout-ice-94961-2.rs stdout ----


- error: values of the type `[u8; 18446744073709551615]` are too big for the current architecture
+ error: values of the type `[u8; 4294967295]` are too big for the current architecture
3 error: aborting due to previous error
4 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/debuginfo/debuginfo-type-name-layout-ice-94961-2/debuginfo-type-name-layout-ice-94961-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args debuginfo/debuginfo-type-name-layout-ice-94961-2.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/debuginfo/debuginfo-type-name-layout-ice-94961-2.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/debuginfo/debuginfo-type-name-layout-ice-94961-2" "-A" "unused" "-Crpath" "-O" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-C" "debuginfo=2" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/debuginfo/debuginfo-type-name-layout-ice-94961-2/auxiliary"
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
stdout: none
--- stderr -------------------------------
error: values of the type `[u8; 4294967295]` are too big for the current architecture
error: aborting due to previous error
------------------------------------------



---- [ui] ui/debuginfo/debuginfo-type-name-layout-ice-94961-1.rs stdout ----


- error: values of the type `[u8; 18446744073709551615]` are too big for the current architecture
+ error: values of the type `[u8; 4294967295]` are too big for the current architecture
3 error: aborting due to previous error
4 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/debuginfo/debuginfo-type-name-layout-ice-94961-1/debuginfo-type-name-layout-ice-94961-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args debuginfo/debuginfo-type-name-layout-ice-94961-1.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/debuginfo/debuginfo-type-name-layout-ice-94961-1.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/debuginfo/debuginfo-type-name-layout-ice-94961-1" "-A" "unused" "-Crpath" "-O" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-C" "debuginfo=2" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/debuginfo/debuginfo-type-name-layout-ice-94961-1/auxiliary"
stdout: none
--- stderr -------------------------------
error: values of the type `[u8; 4294967295]` are too big for the current architecture
error: aborting due to previous error
------------------------------------------




failures:
    [ui] ui/debuginfo/debuginfo-type-name-layout-ice-94961-1.rs
    [ui] ui/debuginfo/debuginfo-type-name-layout-ice-94961-2.rs
test result: FAILED. 12569 passed; 2 failed; 170 ignored; 0 measured; 0 filtered out; finished in 77.29s

Build completed unsuccessfully in 0:01:19

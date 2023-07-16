plain
test [run-make] run-make/rustdoc-themes ... ok
test [run-make] run-make/emit-shared-files ... ok
test [run-make] run-make/rustdoc-determinism ... ok
test [run-make] run-make/thumb-none-cortex-m ... ok
Some tests failed in compiletest suite=run-make mode=run-make host=x86_64-unknown-linux-gnu target=thumbv6m-none-eabi

failures:


---- [run-make] run-make/exit-code stdout ----
error: make failed
status: exit status: 2
command: "make"
stdout:
stdout:
------------------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/exit-code/exit-code:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/exit-code/exit-code -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/exit-code/exit-code  -Clinker='arm-none-eabi-gcc' success.rs; [ $? -eq 0 ]
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
error: linking with `arm-none-eabi-gcc` failed: exit status: 1
  |
  = note: "arm-none-eabi-gcc" "-m64" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/exit-code/exit-code/success.success.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/exit-code/exit-code/success.success.7rcbfp3g-cgu.1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/exit-code/exit-code/success.success.7rcbfp3g-cgu.2.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/exit-code/exit-code/success.success.7rcbfp3g-cgu.3.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/exit-code/exit-code/success.success.7rcbfp3g-cgu.4.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/exit-code/exit-code/success.success.7rcbfp3g-cgu.5.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/exit-code/exit-code/success.success.7rcbfp3g-cgu.6.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/exit-code/exit-code/success.success.7rcbfp3g-cgu.7.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/exit-code/exit-code/success" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/exit-code/exit-code/success.1teweuli9b3bm1hf.rcgu.o" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/exit-code/exit-code" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--start-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-f85d5edbb21544d8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-787faa8b02fbd963.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-dda4c0b69607e93b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-4b7dae8949ac132c.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-978e97832b309706.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-073b1b693304b876.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-c07f996a53ee6558.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-0ae8ed6a282247d0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-978dd04958b6ebcc.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-14b94bdd9a47d665.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-bff7534e4dfcef6c.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-022f1a0e7cd794ec.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-ff456575f1773ef0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-3aeb407930ebd519.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-6ab1ee6dbc17ad08.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-166dae07beec0398.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-761b290f47712921.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc"
  = note: arm-none-eabi-gcc: error: unrecognized command line option '-m64'

error: aborting due to previous error


make: *** [Makefile:4: all] Error 1
------------------------------------------



---- [run-make] run-make/issue-22131 stdout ----
error: make failed
status: exit status: 2
command: "make"
stdout:
stdout:
------------------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-22131/issue-22131:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-22131/issue-22131 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-22131/issue-22131  -Clinker='arm-none-eabi-gcc' --cfg 'feature="bar"' --crate-type lib foo.rs
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-22131/issue-22131:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib -Clinker='arm-none-eabi-gcc' --test --cfg 'feature="bar"' \
 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-22131/issue-22131 foo.rs |\
 "/checkout/src/etc/cat-and-grep.sh" 'foo.rs - foo (line 1) ... ok'
[[[ begin stdout ]]]
running 1 test
test foo.rs - foo (line 1) ... FAILED

failures:
failures:

---- foo.rs - foo (line 1) stdout ----
error: linking with `arm-none-eabi-gcc` failed: exit status: 1
  |
  = note: "arm-none-eabi-gcc" "-m64" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-22131/issue-22131/rustdoctestVzSbFA/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-22131/issue-22131/rustdoctestVzSbFA/rust_out" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-22131/issue-22131/rustdoctestVzSbFA/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-22131/issue-22131" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-22131/issue-22131/libfoo.rlib" "-Wl,--start-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-f85d5edbb21544d8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-787faa8b02fbd963.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-dda4c0b69607e93b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-4b7dae8949ac132c.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-978e97832b309706.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-073b1b693304b876.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-c07f996a53ee6558.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-0ae8ed6a282247d0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-978dd04958b6ebcc.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-14b94bdd9a47d665.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-bff7534e4dfcef6c.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-022f1a0e7cd794ec.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-ff456575f1773ef0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-3aeb407930ebd519.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-6ab1ee6dbc17ad08.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-166dae07beec0398.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-761b290f47712921.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc"
  = note: arm-none-eabi-gcc: error: unrecognized command line option '-m64'

error: aborting due to previous error

Couldn't compile the test.
Couldn't compile the test.

failures:
    foo.rs - foo (line 1)
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s



[[[ end stdout ]]]
Error: cannot match: foo.rs - foo (line 1) ... ok
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
make: *** [Makefile:5: all] Error 1
------------------------------------------



---- [run-make] run-make/issue-38237 stdout ----
error: make failed
status: exit status: 2
command: "make"
stdout:
stdout:
------------------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-38237/issue-38237:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-38237/issue-38237 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-38237/issue-38237  -Clinker='arm-none-eabi-gcc' foo.rs; LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-38237/issue-38237:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-38237/issue-38237 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-38237/issue-38237  -Clinker='arm-none-eabi-gcc' bar.rs
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-38237/issue-38237:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib -Clinker='arm-none-eabi-gcc' baz.rs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-38237/issue-38237 -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-38237/issue-38237
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
error: linking with `arm-none-eabi-gcc` failed: exit status: 1
  |
  = note: "arm-none-eabi-gcc" "-m64" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-38237/issue-38237/foo.foo.3a1fbbbh-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-38237/issue-38237/foo.foo.3a1fbbbh-cgu.1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-38237/issue-38237/foo.foo.3a1fbbbh-cgu.10.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-38237/issue-38237/foo.foo.3a1fbbbh-cgu.11.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-38237/issue-38237/foo.foo.3a1fbbbh-cgu.12.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-38237/issue-38237/foo.foo.3a1fbbbh-cgu.13.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-38237/issue-38237/foo.foo.3a1fbbbh-cgu.14.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-38237/issue-38237/foo.foo.3a1fbbbh-cgu.15.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-38237/issue-38237/foo.foo.3a1fbbbh-cgu.2.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-38237/issue-38237/foo.foo.3a1fbbbh-cgu.3.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-38237/issue-38237/foo.foo.3a1fbbbh-cgu.4.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-38237/issue-38237/foo.foo.3a1fbbbh-cgu.5.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-38237/issue-38237/foo.foo.3a1fbbbh-cgu.6.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-38237/issue-38237/foo.foo.3a1fbbbh-cgu.7.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-38237/issue-38237/foo.foo.3a1fbbbh-cgu.8.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-38237/issue-38237/foo.foo.3a1fbbbh-cgu.9.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-38237/issue-38237/libfoo.so" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-38237/issue-38237/foo.3gjskz8h815ixnja.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-38237/issue-38237/foo.4a2a78br05n0rjfc.rcgu.o" "-Wl,--gc-sections" "-shared" "-Wl,-zrelro" "-Wl,-znow" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-38237/issue-38237" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libproc_macro-ef17864136089118.rlib" "-Wl,--start-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-f85d5edbb21544d8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-787faa8b02fbd963.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-dda4c0b69607e93b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-4b7dae8949ac132c.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-978e97832b309706.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-073b1b693304b876.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-c07f996a53ee6558.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-0ae8ed6a282247d0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-978dd04958b6ebcc.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-14b94bdd9a47d665.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-bff7534e4dfcef6c.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-022f1a0e7cd794ec.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-ff456575f1773ef0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-3aeb407930ebd519.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-6ab1ee6dbc17ad08.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-166dae07beec0398.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-761b290f47712921.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc"
  = note: arm-none-eabi-gcc: error: unrecognized command line option '-m64'

error: aborting due to previous error

error[E0463]: can't find crate for `foo`
error[E0463]: can't find crate for `foo`
 --> baz.rs:1:1
1 | extern crate foo;
  | ^^^^^^^^^^^^^^^^^ can't find crate

error: aborting due to previous error
error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.
make: *** [Makefile:5: all] Error 1
------------------------------------------



---
test result: FAILED. 16 passed; 3 failed; 14 ignored; 0 measured; 0 filtered out; finished in 18.99s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make" "--stage-id" "stage2-thumbv6m-none-eabi" "--suite" "run-make" "--mode" "run-make" "--target" "thumbv6m-none-eabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "arm-none-eabi-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/thumbv6m-none-eabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--llvm-version" "12.0.0-rust-1.53.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--llvm-bin-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin" "--cc" "arm-none-eabi-gcc" "--cxx" "arm-none-eabi-g++" "--cflags" "-ffunction-sections -fdata-sections -mthumb -march=armv6s-m" "--ar" "arm-none-eabi-ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --host= --target thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf src/test/run-make
Build completed unsuccessfully in 0:14:57

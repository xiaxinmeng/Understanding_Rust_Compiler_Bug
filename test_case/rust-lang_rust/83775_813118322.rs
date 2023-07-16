
---- [run-make] run-make/exit-code stdout ----
error: make failed
status: exit status: 2
------------------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/exit-code/exit-code:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/exit-code/exit-code -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/exit-code/exit-code  -Clinker='arm-none-eabi-gcc' success.rs; [ $? -eq 0 ]
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
error: linking with `arm-none-eabi-gcc` failed: exit status: 1
  |
  = note: "arm-none-eabi-gcc" "-m64" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" # lots more args
  = note: arm-none-eabi-gcc: error: unrecognized command line option '-m64'

plain
test [run-make] src/test/run-make/emit-shared-files ... ok
test [run-make] src/test/run-make/rustdoc-scrape-examples-multiple ... ok
test [run-make] src/test/run-make/rustdoc-scrape-examples-ordering ... ok
test [run-make] src/test/run-make/thumb-none-cortex-m ... ok
Some tests failed in compiletest suite=run-make mode=run-make host=x86_64-unknown-linux-gnu target=thumbv6m-none-eabi

failures:


---- [run-make] src/test/run-make/export-executable-symbols stdout ----
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/export-executable-symbols/export-executable-symbols:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/export-executable-symbols/export-executable-symbols -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/export-executable-symbols/export-executable-symbols  -Clinker='arm-none-eabi-gcc' --crate-type=cdylib foo.rs --target thumbv6m-none-eabi
--- stderr -------------------------------
--- stderr -------------------------------
warning: dropping unsupported crate type `cdylib` for target `thumbv6m-none-eabi`
error[E0463]: can't find crate for `std`
  |
  |
  = note: the `thumbv6m-none-eabi` target may not support the standard library
  = note: `std` is required by `<unknown>` because it does not declare `#![no_std]`
  = help: consider building the standard library from source with `cargo build -Zbuild-std`
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0463`.
For more information about this error, try `rustc --explain E0463`.
make: *** [Makefile:4: all] Error 1



failures:

plain
doc tests for: /checkout/src/doc/rustc/src/platform-support/kmc-solid.md
doc tests for: /checkout/src/doc/rustc/src/platform-support/m68k-unknown-linux-gnu.md
doc tests for: /checkout/src/doc/rustc/src/platform-support/openbsd.md
doc tests for: /checkout/src/doc/rustc/src/platform-support/riscv32imac-unknown-xous-elf.md
doc tests for: /checkout/src/doc/rustc/src/platform-support/unknown-uefi.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/src/doc/rustc/src/platform-support/unknown-uefi.md" "--test-args" ""

stdout ----

running 2 tests
running 2 tests
test /checkout/src/doc/rustc/src/platform-support/unknown-uefi.md - Tier__3::Example__Freestanding (line 183) ... FAILED
test /checkout/src/doc/rustc/src/platform-support/unknown-uefi.md - Tier__3::Example__Hello_World (line 213) ... FAILED
failures:


---- /checkout/src/doc/rustc/src/platform-support/unknown-uefi.md - Tier__3::Example__Freestanding (line 183) stdout ----
error: language item required, but not found: `eh_personality`
  |
  = note: this can occur when a binary crate with `#![no_std]` is compiled for a target where `eh_personality` is defined in the standard library
  = help: you may be able to compile for a target that doesn't need `eh_personality`, specify a target with `--target` or in `.cargo/config`
error: aborting due to previous error

Couldn't compile the test.
Couldn't compile the test.
---- /checkout/src/doc/rustc/src/platform-support/unknown-uefi.md - Tier__3::Example__Hello_World (line 213) stdout ----
error[E0432]: unresolved import `r_efi`
 --> /checkout/src/doc/rustc/src/platform-support/unknown-uefi.md:217:5
  |
5 | use r_efi::efi;
  |     ^^^^^ maybe a missing crate `r_efi`?
  |
  = help: consider adding `extern crate r_efi` to use the `r_efi` crate

error: language item required, but not found: `eh_personality`
  |
  = note: this can occur when a binary crate with `#![no_std]` is compiled for a target where `eh_personality` is defined in the standard library
  = help: you may be able to compile for a target that doesn't need `eh_personality`, specify a target with `--target` or in `.cargo/config`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0432`.
Couldn't compile the test.
Couldn't compile the test.

failures:
    /checkout/src/doc/rustc/src/platform-support/unknown-uefi.md - Tier__3::Example__Freestanding (line 183)
    /checkout/src/doc/rustc/src/platform-support/unknown-uefi.md - Tier__3::Example__Hello_World (line 213)
test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s


stderr ----

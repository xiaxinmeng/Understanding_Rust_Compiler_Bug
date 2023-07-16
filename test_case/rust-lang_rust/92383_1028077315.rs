plain
doc tests for: /checkout/src/doc/rustc/src/platform-support/aarch64-apple-ios-sim.md
doc tests for: /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md" "--test-args" ""

stdout ----

running 4 tests
running 4 tests
test /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md - Tier__3::Building_Rust_programs::Cross_Compilation (line 77) ... FAILED
test /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md - Tier__3::Building_Rust_programs::Test_with_QEMU (line 92) ... FAILED
test /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md - Tier__3::Building_Rust_programs::Cross_Compilation (line 72) ... FAILED
test /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md - Tier__3::Building_Rust_programs::Native_compilation (line 59) ... FAILED
failures:


---- /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md - Tier__3::Building_Rust_programs::Cross_Compilation (line 77) stdout ----
error: unknown start of token: \
 --> /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md:78:51
  |
1 | CC=/opt/tomatoware/arm-soft-mmc/bin/arm-linux-gcc \

error: unknown start of token: \
 --> /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md:79:52
  |
  |
2 | CXX=/opt/tomatoware/arm-soft-mmc/bin/arm-linux-g++ \

error: unknown start of token: \
 --> /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md:80:50
  |
  |
3 | AR=/opt/tomatoware/arm-soft-mmc/bin/arm-linux-ar \

error: unknown start of token: \
 --> /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md:81:99
  |
  |
4 | CARGO_TARGET_ARMV7_UNKNOWN_LINUX_UCLIBCEABI_LINKER=/opt/tomatoware/arm-soft-mmc/bin/arm-linux-gcc \

error[E0762]: unterminated character literal
 --> /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md:82:55
  |
  |
5 | CARGO_TARGET_ARMV7_UNKNOWN_LINUX_UCLIBCEABI_RUSTFLAGS='-Clink-arg=-s -Clink-arg=-Wl,--dynamic-linker=/mmc/lib/ld-uClibc.so.1 -Clink-arg=-...

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0762`.
For more information about this error, try `rustc --explain E0762`.
Couldn't compile the test.
---- /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md - Tier__3::Building_Rust_programs::Test_with_QEMU (line 92) stdout ----
error: unknown start of token: \
 --> /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md:93:51
  |
3 | CC=/opt/tomatoware/arm-soft-mmc/bin/arm-linux-gcc \

error: unknown start of token: \
 --> /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md:94:52
  |
  |
4 | CXX=/opt/tomatoware/arm-soft-mmc/bin/arm-linux-g++ \

error: unknown start of token: \
 --> /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md:95:50
  |
  |
5 | AR=/opt/tomatoware/arm-soft-mmc/bin/arm-linux-ar \

error: unknown start of token: \
 --> /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md:96:99
  |
  |
6 | CARGO_TARGET_ARMV7_UNKNOWN_LINUX_UCLIBCEABI_LINKER=/opt/tomatoware/arm-soft-mmc/bin/arm-linux-gcc \

error: unknown start of token: \
 --> /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md:97:146
  |
  |
7 | ...atoware/arm-soft-mmc/arm-tomatoware-linux-uclibcgnueabi/sysroot/" \

error: expected expression, found `/`
 --> /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md:93:4
  |
  |
3 | CC=/opt/tomatoware/arm-soft-mmc/bin/arm-linux-gcc \
  |    ^ expected expression
error: aborting due to 6 previous errors

Couldn't compile the test.
Couldn't compile the test.
---- /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md - Tier__3::Building_Rust_programs::Cross_Compilation (line 72) stdout ----
error: unknown start of token: \
 --> /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md:73:30
  |
3 | rustup toolchain link stage2 \


error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `toolchain`
 --> /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md:73:8
  |
3 | rustup toolchain link stage2 \
  |        ^^^^^^^^^ expected one of 8 possible tokens
error: aborting due to 2 previous errors

Couldn't compile the test.
Couldn't compile the test.
---- /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md - Tier__3::Building_Rust_programs::Native_compilation (line 59) stdout ----
error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `rust_1`
 --> /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md:60:9
  |
3 | dpkg -i rust_1.xx.x-x_arm.deb
  |         ^^^^^^ expected one of 8 possible tokens
error: aborting due to previous error

Couldn't compile the test.


failures:
    /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md - Tier__3::Building_Rust_programs::Cross_Compilation (line 72)
    /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md - Tier__3::Building_Rust_programs::Cross_Compilation (line 77)
    /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md - Tier__3::Building_Rust_programs::Native_compilation (line 59)
    /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md - Tier__3::Building_Rust_programs::Test_with_QEMU (line 92)
test result: FAILED. 0 passed; 4 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s


stderr ----

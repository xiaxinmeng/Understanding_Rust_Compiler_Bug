plain
doc tests for: /checkout/src/doc/rustc/src/lints/levels.md
doc tests for: /checkout/src/doc/rustc/src/platform-support/aarch64-apple-ios-sim.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/src/doc/rustc/src/platform-support/aarch64-apple-ios-sim.md" "--test-args" ""

stdout ----

running 2 tests
running 2 tests
test /checkout/src/doc/rustc/src/platform-support/aarch64-apple-ios-sim.md - aarch64_apple_ios_sim::Building_Rust_programs (line 51) ... FAILED
test /checkout/src/doc/rustc/src/platform-support/aarch64-apple-ios-sim.md - aarch64_apple_ios_sim::Building_Rust_programs (line 45) ... FAILED
failures:


---- /checkout/src/doc/rustc/src/platform-support/aarch64-apple-ios-sim.md - aarch64_apple_ios_sim::Building_Rust_programs (line 51) stdout ----
error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `build`
 --> /checkout/src/doc/rustc/src/platform-support/aarch64-apple-ios-sim.md:52:7
  |
3 | cargo build -Z build-std --target aarch64-apple-ios-sim
  |       ^^^^^ expected one of 8 possible tokens
error: aborting due to previous error

Couldn't compile the test.
Couldn't compile the test.
---- /checkout/src/doc/rustc/src/platform-support/aarch64-apple-ios-sim.md - aarch64_apple_ios_sim::Building_Rust_programs (line 45) stdout ----
error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `aarch64`
 --> /checkout/src/doc/rustc/src/platform-support/aarch64-apple-ios-sim.md:46:16
  |
3 | rustc --target aarch64-apple-ios-sim your-code.rs
  |                ^^^^^^^ expected one of 8 possible tokens
error: aborting due to previous error

Couldn't compile the test.


failures:
    /checkout/src/doc/rustc/src/platform-support/aarch64-apple-ios-sim.md - aarch64_apple_ios_sim::Building_Rust_programs (line 45)
    /checkout/src/doc/rustc/src/platform-support/aarch64-apple-ios-sim.md - aarch64_apple_ios_sim::Building_Rust_programs (line 51)
test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s


stderr ----

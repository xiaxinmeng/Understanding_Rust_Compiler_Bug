plain
---- src/fmt/mod.rs - fmt::Debug (line 574) stdout ----
error[E0433]: failed to resolve: use of undeclared crate or module `fmt`
 --> src/fmt/mod.rs:575:6
  |
4 | impl fmt::Debug for Point {
  |      ^^^ use of undeclared crate or module `fmt`
error[E0433]: failed to resolve: use of undeclared crate or module `fmt`
 --> src/fmt/mod.rs:576:27
  |
  |
5 |     fn fmt(&self, f: &mut fmt::Formatter <'_>) -> fmt::Result {
  |                           ^^^ use of undeclared crate or module `fmt`
error[E0433]: failed to resolve: use of undeclared crate or module `fmt`
 --> src/fmt/mod.rs:576:51
  |
  |
5 |     fn fmt(&self, f: &mut fmt::Formatter <'_>) -> fmt::Result {
  |                                                   ^^^ use of undeclared crate or module `fmt`
error[E0412]: cannot find type `Point` in this scope
 --> src/fmt/mod.rs:575:21
  |
  |
4 | impl fmt::Debug for Point {

error: aborting due to 4 previous errors

Some errors have detailed explanations: E0412, E0433.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:24:59

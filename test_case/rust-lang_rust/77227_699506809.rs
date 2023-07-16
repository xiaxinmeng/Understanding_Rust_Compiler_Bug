plain
Documenting stage2 std (x86_64-unknown-linux-gnu)
Uplifting stage1 std (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Copying stage2 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
 Documenting core v0.0.0 (/checkout/library/core)
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: InterpErrorInfo { kind: scalar size mismatch: expected 4 bytes but got 8 bytes instead, backtrace: None }', compiler/rustc_middle/src/ty/print/pretty.rs:1019:57

error: internal compiler error: unexpected panic


error: Unrecognized option: 'markdown-css'

error: Could not document `core`.
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name core library/core/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.48.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps -Dwarnings -Winvalid_codeblock_attributes --crate-version '1.48.0-nightly
  (29ff7ae43
  2020-09-26)'` (exit code: 1)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "-Z" "unstable-options" "--resource-suffix" "1.48.0" "--index-page" "/checkout/src/doc/index.md"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
Build completed unsuccessfully in 0:19:38
Build completed unsuccessfully in 0:19:38
== clock drift check ==
  local time: Sat Sep 26 15:00:57 UTC 2020
  network time: Sat, 26 Sep 2020 15:00:57 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (4977) (python)

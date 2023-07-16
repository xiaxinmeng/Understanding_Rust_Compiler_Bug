plain
Successfully built ffb74716f515
Successfully tagged rust-ci:latest
Built container sha256:ffb74716f515ce82e5ae4e23e494919cb2a1ba74ac5957d5e01c8907e48d5c6f
Uploading finished image to https://ci-caches.rust-lang.org/docker/f6bc86eb4a9cdf8ee7fada0150ff022ba52e09a7d507b9ebc7eac13b52c509ffb94b984d1124f5f993be40b565ca80b1f5e39692d38edfff28e636c5574b922c
upload failed: - to s3://rust-lang-ci-sccache2/docker/f6bc86eb4a9cdf8ee7fada0150ff022ba52e09a7d507b9ebc7eac13b52c509ffb94b984d1124f5f993be40b565ca80b1f5e39692d38edfff28e636c5574b922c Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
 Documenting core v0.0.0 (/checkout/library/core)
error: unresolved link to `PanicInfo::location`
 --> library/core/src/panic/location.rs:5:35
  |
5 | /// This structure is created by [`PanicInfo::location()`].
  |                                   ^^^^^^^^^^^^^^^^^^^^^^^ no item named `PanicInfo` in scope
  |
  = note: `-D rustdoc::broken-intra-doc-links` implied by `-D warnings`
error: unresolved link to `catch_unwind`
  --> library/core/src/panic/unwind_safe.rs:15:72
   |
   |
15 | /// purpose of this trait is to encode what types are safe to cross a [`catch_unwind`]
   |                                                                        ^^^^^^^^^^^^^^ no item named `catch_unwind` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `catch_unwind`
   --> library/core/src/panic/unwind_safe.rs:109:17
    |
    |
109 | /// When using [`catch_unwind`] it may be the case that some of the closed over
    |                 ^^^^^^^^^^^^^^ no item named `catch_unwind` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `catch_unwind`
   --> library/core/src/panic/unwind_safe.rs:113:24
    |
    |
113 | /// specific usage of [`catch_unwind`] if unwind safety is specifically taken into
    |                        ^^^^^^^^^^^^^^ no item named `catch_unwind` in scope
    |
    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `PanicInfo::location`
 --> library/core/src/panic/location.rs:5:35
  |
  |
5 | /// This structure is created by [`PanicInfo::location()`].
  |                                   ^^^^^^^^^^^^^^^^^^^^^^^ no item named `PanicInfo` in scope
error: aborting due to 5 previous errors

error: could not document `core`


Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name core library/core/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.53.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --cfg=bootstrap -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.53.0-nightly
  (a9176affe
  2021-04-28)'` (exit code: 1)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "-Z" "unstable-options" "--resource-suffix" "1.53.0" "--index-page" "/checkout/src/doc/index.md"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc --stage 0 library/std
Build completed unsuccessfully in 0:00:14

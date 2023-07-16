plain
 Documenting core v0.0.0 (/checkout/library/core)
error: functions cannot be const-stable if they are unstable
  --> library/core/src/../../stdarch/crates/core_arch/src/wasm32/simd128.rs:58:17
   |
58 | /                 const fn v128(self) -> v128 {
59 | |                     unsafe { mem::transmute(self) }
   | |_________________^
...
...
66 | / conversions! {
67 | |     (as_u8x16 = simd::u8x16)
68 | |     (as_u16x8 = simd::u16x8)
69 | |     (as_u32x4 = simd::u32x4)
...  |
76 | |     (as_f64x2 = simd::f64x2)
   | |_- in this macro invocation
   |
   |
   = note: `#[deny(rustc::incompatible_stability)]` on by default
   = note: this error originates in the macro `conversions` (in Nightly builds, run with -Z macro-backtrace for more info)
error: could not document `core`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name core library/core/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.58.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps -Zsymbol-mangling-version=legacy -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.58.0-nightly
  (347467193
  2021-10-28)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit status: 1)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "-Zskip-rustdoc-fingerprint" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "-Z" "unstable-options" "--resource-suffix" "1.58.0" "--index-page" "/checkout/src/doc/index.md"


Build completed unsuccessfully in 0:32:25

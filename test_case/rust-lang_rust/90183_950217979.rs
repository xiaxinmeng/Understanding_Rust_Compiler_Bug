plain
 Documenting std v0.0.0 (/checkout/library/std)
    Finished release [optimized] target(s) in 17.35s
   Compiling std v0.0.0 (/checkout/library/std)
 Documenting proc_macro v0.0.0 (/checkout/library/proc_macro)
error[E0773]: attempted to define built-in macro more than once
     |
1223 | /     macro_rules! cfg {
1223 | /     macro_rules! cfg {
1224 | |         ($($cfg:tt)*) => {
1226 | |         };
1227 | |     }
     | |_____^
     |
     |
note: previously defined here
    --> /checkout/library/core/src/macros/mod.rs:1223:5
     |
1223 | /     macro_rules! cfg {
1224 | |         ($($cfg:tt)*) => {
1226 | |         };
1227 | |     }
     | |_____^

---
For more information about this error, try `rustc --explain E0773`.
error: could not document `proc_macro`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name proc_macro library/proc_macro/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.58.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern std=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libstd-e89a759f9eb7ec65.rmeta -Zsymbol-mangling-version=legacy -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.58.0-nightly
  (5a75baefb
  2021-10-23)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit status: 1)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "proc_macro" "-Zskip-rustdoc-fingerprint" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "-Z" "unstable-options" "--resource-suffix" "1.58.0" "--index-page" "/checkout/src/doc/index.md"


Build completed unsuccessfully in 0:25:03

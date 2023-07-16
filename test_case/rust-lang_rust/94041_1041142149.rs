plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
 Documenting std v0.0.0 (/checkout/library/std)
    Finished release [optimized] target(s) in 10.10s
    Checking std v0.0.0 (/checkout/library/std)
 Documenting proc_macro v0.0.0 (/checkout/library/proc_macro)
error[E0773]: attempted to define built-in macro more than once
     |
1306 | /     macro_rules! cfg {
1306 | /     macro_rules! cfg {
1307 | |         ($($cfg:tt)*) => {
1309 | |         };
1310 | |     }
     | |_____^
     |
     |
note: previously defined here
    --> /checkout/library/core/src/macros/mod.rs:1306:5
     |
1306 | /     macro_rules! cfg {
1307 | |         ($($cfg:tt)*) => {
1309 | |         };
1310 | |     }
     | |_____^

---
For more information about this error, try `rustc --explain E0773`.
error: could not document `proc_macro`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name proc_macro library/proc_macro/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi,future-incompat --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.60.0 --index-page /checkout/src/doc/index.md -C metadata=030f61473e41e060 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps --extern std=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libstd-8ecdf0b3fdf1770d.rmeta --cfg=bootstrap -Csymbol-mangling-version=legacy -Zunstable-options -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.60.0-nightly
  (29f6d4c13
  2022-02-16)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit status: 1)

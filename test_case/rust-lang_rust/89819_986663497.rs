plain

######################################################################## 100.0%
extracting /checkout/obj/build/cache/2021-11-30/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
    Updating crates.io index
    Updating git repository `https://github.com/davidtwco/thorin.git`
---
   Compiling thiserror-impl v1.0.30
    Checking chalk-ir v0.55.0
    Checking tracing v0.1.29
    Checking thiserror v1.0.30
    Checking thorin v0.1.0 (https://github.com/davidtwco/thorin.git#b0d5ca9a)
    Checking rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
    Checking rustc_arena v0.0.0 (/checkout/compiler/rustc_arena)
    Checking rustc_type_ir v0.0.0 (/checkout/compiler/rustc_type_ir)
    Checking rustc_span v0.0.0 (/checkout/compiler/rustc_span)
---
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
   Compiling thiserror-impl v1.0.30
    Checking chalk-ir v0.55.0
    Checking tracing v0.1.29
    Checking thiserror v1.0.30
    Checking thorin v0.1.0 (https://github.com/davidtwco/thorin.git#b0d5ca9a)
    Checking rustc_index v0.0.0 (/checkout/compiler/rustc_index)
error[E0433]: failed to resolve: could not find `unix` in `os`
  --> /cargo/git/checkouts/thorin-c1d683161cc6bbfc/b0d5ca9/thorin/src/util.rs:13:9
13 |     os::unix::fs::FileTypeExt,
13 |     os::unix::fs::FileTypeExt,
   |         ^^^^ could not find `unix` in `os`

error[E0599]: no method named `is_fifo` found for struct `FileType` in the current scope
  --> /cargo/git/checkouts/thorin-c1d683161cc6bbfc/b0d5ca9/thorin/src/util.rs:62:41
   |
62 |         if file.metadata()?.file_type().is_fifo() {
   |                                         ^^^^^^^ help: there is an associated function with a similar name: `is_file`
    Checking rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
Some errors have detailed explanations: E0433, E0599.
For more information about an error, try `rustc --explain E0433`.
For more information about an error, try `rustc --explain E0433`.
error: could not compile `thorin` due to 2 previous errors
error: build failed
Build completed unsuccessfully in 0:00:48

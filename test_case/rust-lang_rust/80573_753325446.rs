plain
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
    Checking cargo_metadata v0.12.0
    Checking rustfix v0.5.1
    Checking rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
    Checking clippy_lints v0.0.212 (/checkout/src/tools/clippy/clippy_lints)
error: Prefer FxHashMap over HashMap, it has better performance
   |
36 |     use std::collections::HashMap;
36 |     use std::collections::HashMap;
   |                           ^^^^^^^ help: use: `FxHashMap`
   |
   = note: `-D rustc::default-hash-types` implied by `-D warnings`
   = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary

error: Prefer FxHashMap over HashMap, it has better performance
   |
   |
39 |     let mut crates: HashMap<&str, PathBuf> = HashMap::with_capacity(CRATES.len());
   |                     ^^^^^^^ help: use: `FxHashMap`
   |
   = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary

error: Prefer FxHashMap over HashMap, it has better performance
   |
   |
39 |     let mut crates: HashMap<&str, PathBuf> = HashMap::with_capacity(CRATES.len());
   |                                              ^^^^^^^ help: use: `FxHashMap`
   |
   = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary
error: aborting due to 3 previous errors

error: could not compile `clippy`


To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/clippy/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--all-targets" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:03:12

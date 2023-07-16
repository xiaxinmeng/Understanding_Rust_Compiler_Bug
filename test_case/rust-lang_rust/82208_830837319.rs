plain
    Checking same-file v1.0.6
    Checking ansi_term v0.11.0
    Checking smallvec v1.6.1
   Compiling rustfmt-nightly v1.4.37 (/checkout/src/tools/rustfmt)
    Checking yansi-term v0.1.2
    Checking unicode_categories v0.1.1
    Checking diff v0.1.12
   Compiling proc-macro2 v1.0.24
   Compiling proc-macro-error-attr v1.0.4
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
   Compiling rustfmt-config_proc_macro v0.2.0 (/checkout/src/tools/rustfmt/config_proc_macro)
    Checking humantime v1.3.0
    Checking semver v0.9.0
    Checking cargo_metadata v0.8.2
error: found crates (`serde_derive` and `serde_derive`) with colliding StableCrateId values.
    |
162 | extern crate serde_derive;
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^


error: aborting due to previous error

error: could not compile `cargo_metadata`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustfmt/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--all-targets" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:02:58

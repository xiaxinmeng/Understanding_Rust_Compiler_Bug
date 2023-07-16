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
    Checking rustfix v0.5.1
    Checking clippy_lints v0.1.52 (/checkout/src/tools/clippy/clippy_lints)
    Checking rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
    Finished release [optimized] target(s) in 29.45s
Checking stage0 rustfmt artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
error: package `rustfmt-nightly v1.4.36 (/checkout/src/tools/rustfmt)` does not have a dependency named `rustc-workspace-hack`
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustfmt/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--all-targets" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:03:21

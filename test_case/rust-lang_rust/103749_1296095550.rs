plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
 ) {
-
     macro_rules! emit_diag {
         (
             $lint:expr,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_traits/src/chalk/lowering.rs" "/checkout/compiler/rustc_mir_build/src/thir/pattern/check_match.rs" "/checkout/compiler/rustc_mir_build/src/thir/cx/block.rs" "/checkout/compiler/rustc_mir_build/src/thir/pattern/deconstruct_pat.rs" "/checkout/compiler/rustc_mir_build/src/thir/cx/expr.rs" "/checkout/compiler/rustc_mir_build/src/thir/pattern/usefulness.rs" "/checkout/compiler/rustc_mir_build/src/thir/cx/mod.rs" "/checkout/compiler/rustc_mir_build/src/thir/util.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.

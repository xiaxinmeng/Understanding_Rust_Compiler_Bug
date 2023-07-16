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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_codegen_ssa/src/base.rs at line 783:
         //
         // In order to get this left-to-right dependency ordering, we use the reverse
         // postorder of all crates putting the leaves at the right-most positions.
-        let used_crates = tcx.postorder_cnums(())
+        let used_crates = tcx
+            .postorder_cnums(())
             .iter()
             .rev()
             .copied()
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_ssa/src/back/write.rs" "/checkout/compiler/rustc_codegen_ssa/src/traits/write.rs" "/checkout/compiler/rustc_codegen_ssa/src/traits/builder.rs" "/checkout/compiler/rustc_codegen_ssa/src/traits/consts.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/rpath.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/linker.rs" "/checkout/compiler/rustc_codegen_ssa/src/base.rs" "/checkout/compiler/rustc_codegen_ssa/src/traits/debuginfo.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.

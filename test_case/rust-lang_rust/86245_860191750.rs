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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_mir/src/interpret/traits.rs at line 139:
         if args.len() != 1 {
             throw_ub!(InvalidVtableDropFn(fn_sig));
         }
-        let ty = args[0].builtin_deref(true).ok_or_else(|| err_ub!(InvalidVtableDropFn(fn_sig)))?.ty;
+        let ty =
+            args[0].builtin_deref(true).ok_or_else(|| err_ub!(InvalidVtableDropFn(fn_sig)))?.ty;
         Ok((drop_instance, ty))
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir/src/transform/inline.rs" "/checkout/compiler/rustc_span/src/symbol.rs" "/checkout/compiler/rustc_mir/src/util/spanview.rs" "/checkout/compiler/rustc_mir/src/interpret/validity.rs" "/checkout/compiler/rustc_mir/src/interpret/operand.rs" "/checkout/compiler/rustc_mir/src/interpret/place.rs" "/checkout/compiler/rustc_mir/src/interpret/traits.rs" "/checkout/compiler/rustc_span/src/hygiene.rs"` failed.
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.

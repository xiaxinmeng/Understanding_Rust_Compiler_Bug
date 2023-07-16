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

######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-06-29/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/compiler/rustc_builtin_macros/src/deriving/generic/mod.rs at line 1459:
             .iter()
             .iter()
             .map(|prefix| {
-                let pieces_iter = struct_def
-                    .fields()
-                    .iter()
-                    .enumerate()
-                    .map(|(i, struct_field)| {
+                let pieces_iter =
+                    struct_def.fields().iter().enumerate().map(|(i, struct_field)| {
                         let sp = struct_field.span.with_ctxt(self.span.ctxt());
                         let binding_mode = if use_temporaries {
                             ast::BindingMode::ByValue(ast::Mutability::Not)
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_builtin_macros/src/deriving/debug.rs" "/checkout/compiler/rustc_builtin_macros/src/assert.rs" "/checkout/compiler/rustc_builtin_macros/src/deriving/generic/mod.rs" "/checkout/compiler/rustc_builtin_macros/src/compile_error.rs" "/checkout/compiler/rustc_builtin_macros/src/deriving/generic/ty.rs" "/checkout/compiler/rustc_builtin_macros/src/assert/context.rs" "/checkout/compiler/rustc_builtin_macros/src/standard_library_imports.rs" "/checkout/compiler/rustc_ty_utils/src/representability.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:167:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:167:17
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', /cargo/registry/src/github.com-1ecc6299db9ec823/ignore-0.4.18/src/walk.rs:1302:31

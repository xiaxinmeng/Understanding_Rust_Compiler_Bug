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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_mir_build/src/thir/constant.rs at line 46:
         (ast::LitKind::Int(n, _), ty::Uint(_)) | (ast::LitKind::Int(n, _), ty::Int(_)) => {
             trunc(if neg { (*n as i128).overflowing_neg().0 as u128 } else { *n })?
         }
-        (ast::LitKind::Float(n, _), ty::Float(fty)) => parse_float(*n, *fty, neg).ok_or(LitToConstError::Reported)?,
+        (ast::LitKind::Float(n, _), ty::Float(fty)) => {
+            parse_float(*n, *fty, neg).ok_or(LitToConstError::Reported)?
+        }
         (ast::LitKind::Bool(b), ty::Bool) => ConstValue::Scalar(Scalar::from_bool(*b)),
         (ast::LitKind::Char(c), ty::Char) => ConstValue::Scalar(Scalar::from_char(*c)),
         (ast::LitKind::Err(_), _) => return Err(LitToConstError::Reported),
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir_build/src/thir/constant.rs" "/checkout/compiler/rustc_mir_build/src/build/expr/stmt.rs" "/checkout/compiler/rustc_mir_build/src/build/expr/into.rs" "/checkout/compiler/rustc_mir_build/src/build/expr/as_place.rs" "/checkout/compiler/rustc_span/src/tests.rs" "/checkout/compiler/rustc_mir_build/src/lints.rs" "/checkout/compiler/rustc_span/src/lev_distance.rs" "/checkout/compiler/rustc_span/src/analyze_source_file.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.

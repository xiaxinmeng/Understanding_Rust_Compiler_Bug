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
Diff in /checkout/compiler/rustc_const_eval/src/interpret/operand.rs at line 281:
         // FIXME: can these ever have Scalar ABI?
         ty::Closure(..) | ty::Generator(..) => false,
         // Types that don't have scalar layout to begin with.
-        ty::Array(..) | ty::Slice(..) | ty::Str | ty::Dynamic(..) | ty::Foreign(..) => {
-        }
-        }
+        ty::Array(..) | ty::Slice(..) | ty::Str | ty::Dynamic(..) | ty::Foreign(..) => false,
         // Types we should not uusally see here, but when called from CTFE op_to_const these can
         // actually happen.
         ty::Error(_)
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_const_eval/src/interpret/operand.rs" "/checkout/compiler/rustc_const_eval/src/const_eval/error.rs" "/checkout/compiler/rustc_const_eval/src/util/mod.rs" "/checkout/compiler/rustc_const_eval/src/const_eval/mod.rs" "/checkout/compiler/rustc_const_eval/src/util/alignment.rs" "/checkout/compiler/rustc_const_eval/src/const_eval/fn_queries.rs" "/checkout/compiler/rustc_const_eval/src/util/find_self_call.rs" "/checkout/compiler/rustc_const_eval/src/interpret/util.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.

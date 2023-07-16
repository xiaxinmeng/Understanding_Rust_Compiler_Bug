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
Diff in /checkout/compiler/rustc_const_eval/src/interpret/validity.rs at line 547:
                 if M::enforce_number_no_provenance(self.ecx) {
                     // As a special exception we *do* match on a `Scalar` here, since we truly want
                     // to know its underlying representation (and *not* cast it to an integer).
-                    let is_ptr =
-                        value.check_init().map_or(false, |v| matches!(v, Scalar::Ptr(..)));
+                    let is_ptr = value.check_init().map_or(false, |v| matches!(v, Scalar::Ptr(..)));
                     if is_ptr {
                         throw_validation_failure!(self.path,
                             { "{:x}", value } expected { "plain (non-pointer) bytes" }
Build completed unsuccessfully in 0:00:13
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_const_eval/src/interpret/visitor.rs" "/checkout/compiler/rustc_const_eval/src/interpret/machine.rs" "/checkout/compiler/rustc_const_eval/src/interpret/memory.rs" "/checkout/compiler/rustc_const_eval/src/interpret/operator.rs" "/checkout/compiler/rustc_const_eval/src/interpret/place.rs" "/checkout/compiler/rustc_const_eval/src/interpret/intrinsics.rs" "/checkout/compiler/rustc_const_eval/src/interpret/validity.rs" "/checkout/compiler/rustc_incremental/src/persist/work_product.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.

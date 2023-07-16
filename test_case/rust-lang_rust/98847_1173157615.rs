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
############################################                              61.5%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-06-29/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/compiler/rustc_const_eval/src/interpret/validity.rs at line 596:
             ty::Adt(def, ..) if def.is_box() => {
                 // Box is special, very special.
                 assert_eq!(value.layout.fields.count(), 2, "`Box` must have exactly 2 fields");
-                let (unique_ptr, alloc) = (self.ecx.operand_field(value, 0)?, self.ecx.operand_field(value, 1)?);
+                let (unique_ptr, alloc) =
+                    (self.ecx.operand_field(value, 0)?, self.ecx.operand_field(value, 1)?);
                 // Unfortunately there is some type junk in the way here: `unique_ptr` is a `Unique`...
                 assert_eq!(unique_ptr.layout.fields.count(), 2);
-                let (nonnull_ptr, phantom) = (self.ecx.operand_field(value, 0)?, self.ecx.operand_field(value, 1)?);
+                let (nonnull_ptr, phantom) =
+                    (self.ecx.operand_field(value, 0)?, self.ecx.operand_field(value, 1)?);
                 assert!(phantom.layout.ty.ty_adt_def().is_some_and(|adt| adt.is_phantom_data()));
                 // ... that contains a `NonNull`...
                 assert_eq!(nonnull_ptr.layout.fields.count(), 1);
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_const_eval/src/interpret/validity.rs" "/checkout/compiler/rustc_const_eval/src/const_eval/eval_queries.rs" "/checkout/compiler/rustc_const_eval/src/interpret/place.rs" "/checkout/compiler/rustc_const_eval/src/const_eval/fn_queries.rs" "/checkout/compiler/rustc_const_eval/src/interpret/util.rs" "/checkout/compiler/rustc_const_eval/src/const_eval/error.rs" "/checkout/compiler/rustc_const_eval/src/interpret/operator.rs" "/checkout/compiler/rustc_const_eval/src/interpret/terminator.rs"` failed.
Build completed unsuccessfully in 0:00:15
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.

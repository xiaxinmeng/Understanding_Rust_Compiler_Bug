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
Diff in /checkout/compiler/rustc_lint/src/enum_intrinsics_non_enums.rs at line 67:
                     "the return value of `mem::variant_count` is \
                         unspecified when called with a non-enum type",
                 )
-                .note(
-                    &format!(
-                        "the type parameter of `variant_count` should \
+                .note(&format!(
+                    "the type parameter of `variant_count` should \
                             be an enum, but it was instantiated with \
                             the type `{}`, which is not an enum.",
-                        ty_param,
-                )
+                    ty_param,
+                ))
+                ))
                 .emit();
         });
     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_lint/src/passes.rs" "/checkout/compiler/rustc_lint/src/early.rs" "/checkout/compiler/rustc_lint/src/redundant_semicolon.rs" "/checkout/compiler/rustc_driver/src/lib.rs" "/checkout/compiler/rustc_lint/src/enum_intrinsics_non_enums.rs" "/checkout/compiler/rustc_driver/src/args.rs" "/checkout/compiler/rustc_lint/src/levels.rs" "/checkout/compiler/rustc_lint/src/traits.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:13

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
+                        + 1
                 })
                 .sum()
         }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_ty_utils/src/errors.rs" "/checkout/compiler/rustc_ty_utils/src/needs_drop.rs" "/checkout/compiler/rustc_ty_utils/src/layout_sanity_check.rs" "/checkout/compiler/rustc_ty_utils/src/lib.rs" "/checkout/compiler/rustc_ty_utils/src/consts.rs" "/checkout/compiler/rustc_ty_utils/src/ty.rs" "/checkout/compiler/rustc_ty_utils/src/representability.rs" "/checkout/compiler/rustc_error_codes/src/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.

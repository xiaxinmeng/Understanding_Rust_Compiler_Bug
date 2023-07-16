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
 
+use itertools::Itertools;
 use std::borrow::Borrow;
 use std::cell::OnceCell;
 use std::collections::BTreeSet;
Diff in /checkout/compiler/rustc_codegen_ssa/src/back/link.rs at line 47:
 use std::path::{Path, PathBuf};
 use std::process::{ExitStatus, Output, Stdio};
 use std::{env, fmt, fs, io, mem, str};
 
 
 pub fn ensure_removed(diag_handler: &Handler, path: &Path) {
     if let Err(e) = fs::remove_file(path) {
Diff in /checkout/compiler/rustc_codegen_ssa/src/back/link.rs at line 220:
             let (ty1, list1) = combination[0];
             let (ty2, list2) = combination[1];
             if list1 != list2 {
-                return Err(format!("{ty1:?} and {ty2:?} do not have equivalent dependency formats (`{list1:?}` vs `{list2:?}`)"));
+                return Err(format!(
+                    "{ty1:?} and {ty2:?} do not have equivalent dependency formats (`{list1:?}` vs `{list2:?}`)"
             }
         }
     }
     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_ssa/src/back/mod.rs" "/checkout/compiler/rustc_codegen_ssa/src/debuginfo/type_names.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/metadata.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/link.rs" "/checkout/compiler/rustc_codegen_ssa/src/back/command.rs" "/checkout/compiler/rustc_codegen_ssa/src/mono_item.rs" "/checkout/compiler/rustc_codegen_ssa/src/meth.rs" "/checkout/compiler/rustc_codegen_ssa/src/debuginfo/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.

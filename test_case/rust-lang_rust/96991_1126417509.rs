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
Diff in /checkout/compiler/rustc_mir_dataflow/src/impls/single_enum_variant.rs at line 94:
                 Rvalue::Ref(_, BorrowKind::Mut { .. }, rhs)
                 | Rvalue::AddressOf(Mutability::Mut, rhs),
             )) => {
-                if !rhs.ty(self.body, self.tcx).ty.is_enum() { return }
+                if !rhs.ty(self.body, self.tcx).ty.is_enum() {
+                }
+                }
                 let Some(local) = rhs.local_or_deref_local() else { return };
                 state[local] = FlatSet::Top;
             }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir_dataflow/src/impls/init_locals.rs" "/checkout/compiler/rustc_mir_dataflow/src/impls/single_enum_variant.rs" "/checkout/compiler/rustc_mir_dataflow/src/elaborate_drops.rs" "/checkout/compiler/rustc_query_impl/src/on_disk_cache.rs" "/checkout/compiler/rustc_mir_dataflow/src/impls/mod.rs" "/checkout/compiler/rustc_mir_dataflow/src/impls/storage_liveness.rs" "/checkout/compiler/rustc_mir_dataflow/src/impls/borrowed_locals.rs" "/checkout/compiler/rustc_mir_dataflow/src/framework/cursor.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.

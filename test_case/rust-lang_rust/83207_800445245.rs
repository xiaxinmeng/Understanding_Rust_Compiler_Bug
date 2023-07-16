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
Diff in /checkout/compiler/rustc_mir/src/util/pretty.rs at line 453:
                     ConstantKind::Ty(literal) => self.push(&format!("+ literal: {:?}", literal)),
                     ConstantKind::Val(val, ty) => {
                         // To keep the diffs small, we render this almost like we render ty::Const
-                        self.push(&format!("+ literal: Const {{ ty: {}, val: Value({:?}) }}", ty, val))
+                        self.push(&format!(
+                            "+ literal: Const {{ ty: {}, val: Value({:?}) }}",
+                            ty, val
                     }
                 }
             }
             }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir/src/util/mod.rs" "/checkout/compiler/rustc_mir/src/util/elaborate_drops.rs" "/checkout/compiler/rustc_mir/src/util/collect_writes.rs" "/checkout/compiler/rustc_mir/src/util/pretty.rs" "/checkout/compiler/rustc_mir/src/util/spanview.rs" "/checkout/compiler/rustc_mir/src/util/generic_graphviz.rs" "/checkout/compiler/rustc_mir/src/util/generic_graph.rs" "/checkout/compiler/rustc_mir/src/util/find_self_call.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:16

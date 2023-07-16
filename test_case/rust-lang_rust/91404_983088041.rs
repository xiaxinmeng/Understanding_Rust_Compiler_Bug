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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_resolve/src/lib.rs at line 1430:
     }
 
     pub fn next_node_id(&mut self) -> NodeId {
-        let next = self
-            .next_node_id
-            .as_u32()
-            .checked_add(1)
-            .expect("input too large; ran out of NodeIds");
+        let next =
+            self.next_node_id.as_u32().checked_add(1).expect("input too large; ran out of NodeIds");
         self.next_node_id = ast::NodeId::from_u32(next);
         self.next_node_id
     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_resolve/src/check_unused.rs" "/checkout/compiler/rustc_resolve/src/macros.rs" "/checkout/compiler/rustc_resolve/src/def_collector.rs" "/checkout/compiler/rustc_resolve/src/lib.rs" "/checkout/compiler/rustc_resolve/src/late.rs" "/checkout/compiler/rustc_resolve/src/diagnostics.rs" "/checkout/compiler/rustc_resolve/src/imports.rs" "/checkout/compiler/rustc_monomorphize/src/partitioning/merging.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.

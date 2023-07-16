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

######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-05-20/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/compiler/rustc_query_system/src/dep_graph/serialized.rs at line 139:
 
     pub fn node_count(&self) -> usize {
     pub fn node_count(&self) -> usize {
-        if let Some(ref nodes) = self.nodes {
-            nodes.len()
-            0
-        }
-        }
+        if let Some(ref nodes) = self.nodes { nodes.len() } else { 0 }
 
 
     #[instrument(level = "debug", skip(mmap))]
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_query_system/src/ich/hcx.rs" "/checkout/compiler/rustc_query_system/src/lib.rs" "/checkout/compiler/rustc_query_system/src/dep_graph/debug.rs" "/checkout/compiler/rustc_query_system/src/dep_graph/graph.rs" "/checkout/compiler/rustc_query_system/src/dep_graph/serialized.rs" "/checkout/compiler/rustc_query_system/src/dep_graph/query.rs" "/checkout/compiler/rustc_query_system/src/dep_graph/mod.rs" "/checkout/compiler/rustc_query_system/src/ich/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.

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
Diff in /checkout/src/bootstrap/builder.rs at line 389:
     }
 
     pub fn suite_path(mut self, suite: &str) -> Self {
-        self.paths.insert(PathSet::Suite(TaskPath {
-            path: suite.into(),
-            kind: Some(self.kind.into()),
+        self.paths
+        self.paths
+            .insert(PathSet::Suite(TaskPath { path: suite.into(), kind: Some(self.kind.into()) }));
     }
 
Diff in /checkout/src/bootstrap/builder.rs at line 1749:
Diff in /checkout/src/bootstrap/builder.rs at line 1749:
             if should_run.paths.iter().any(|s| s.has(path, Some(desc.kind)))
                 && !desc.is_excluded(
                     self,
-                    &PathSet::Suite(TaskPath {
-                        path: path.clone(),
-                        kind: Some(desc.kind.into()),
-                    }),
+                    &PathSet::Suite(TaskPath { path: path.clone(), kind: Some(desc.kind.into()) }),
             {
                 return true;
                 return true;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/bin/main.rs" "/checkout/src/bootstrap/build.rs" "/checkout/src/bootstrap/builder.rs" "/checkout/src/bootstrap/tool.rs" "/checkout/src/bootstrap/cc_detect.rs" "/checkout/src/build_helper/lib.rs" "/checkout/compiler/rustc_feature/src/removed.rs" "/checkout/src/bootstrap/bin/llvm-config-wrapper.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.

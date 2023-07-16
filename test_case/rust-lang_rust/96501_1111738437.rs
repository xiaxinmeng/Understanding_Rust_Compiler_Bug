plain
Successfully built ad96341a6053
Successfully tagged rust-ci:latest
Built container sha256:ad96341a6053f2d80f8e972aa34952703ff0012ca423d789b189d97bace35c9c
Uploading finished image to https://ci-caches.rust-lang.org/docker/c165c0dbe07f979c65839814f65d46fc3b20640fec5c6162fa0d12eccd662a5501f008e9d9c1953dc5ee940f259fa67db4f7e378e34b63c10ab0f0e1d88ab56a
upload failed: - to s3://rust-lang-ci-sccache2/docker/c165c0dbe07f979c65839814f65d46fc3b20640fec5c6162fa0d12eccd662a5501f008e9d9c1953dc5ee940f259fa67db4f7e378e34b63c10ab0f0e1d88ab56a Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
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
Diff in /checkout/src/bootstrap/builder/tests.rs at line 36:
 
 #[test]
 fn test_intersection() {
-    let set = PathSet::Set(["library/core", "library/alloc", "library/std"].into_iter().map(TaskPath::parse).collect());
-    let subset = set.intersection(&[Path::new("library/core"), Path::new("library/alloc"), Path::new("library/stdarch")], None);
-    assert_eq!(subset, PathSet::Set(["library/core", "library/alloc"].into_iter().map(TaskPath::parse).collect()));
+    let set = PathSet::Set(
+        ["library/core", "library/alloc", "library/std"].into_iter().map(TaskPath::parse).collect(),
+    let subset = set.intersection(
+    let subset = set.intersection(
+        &[Path::new("library/core"), Path::new("library/alloc"), Path::new("library/stdarch")],
+    );
+    assert_eq!(
+        subset,
+        subset,
+        PathSet::Set(["library/core", "library/alloc"].into_iter().map(TaskPath::parse).collect())
 }
 
 #[test]
Diff in /checkout/src/bootstrap/builder.rs at line 184:
Diff in /checkout/src/bootstrap/builder.rs at line 184:
             false
         };
         match self {
-            PathSet::Set(set) => PathSet::Set(
-                set.iter()
-                    .filter(|&p| check(p))
-                    .cloned()
-                    .collect(),
-            ),
+            PathSet::Set(set) => PathSet::Set(set.iter().filter(|&p| check(p)).cloned().collect()),
             PathSet::Suite(suite) => {
                 if check(suite) {
Diff in /checkout/src/bootstrap/builder.rs at line 294:
         }
 
 
         // strip CurDir prefix if present
-        let mut paths: Vec<_> = paths.into_iter().map(|p| p.strip_prefix(".").unwrap_or(p)).collect();
+        let mut paths: Vec<_> =
+            paths.into_iter().map(|p| p.strip_prefix(".").unwrap_or(p)).collect();
         // Handle all test suite paths.
         // Handle all test suite paths.
         // (This is separate from the loop below to avoid having to handle multiple paths in `is_suite_path` somehow.)
Diff in /checkout/src/bootstrap/builder.rs at line 308:
         });
 
-        if paths.is_empty() { return; }
+        if paths.is_empty() {
+        if paths.is_empty() {
+            return;
+        }
 
         // Handle all PathSets.
         for (desc, should_run) in v.iter().zip(&should_runs) {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/proc_macro/src/bridge/server.rs" "/checkout/src/bootstrap/builder.rs" "/checkout/src/bootstrap/sanity.rs" "/checkout/library/proc_macro/src/bridge/rpc.rs" "/checkout/src/bootstrap/test.rs" "/checkout/src/bootstrap/dist.rs" "/checkout/src/bootstrap/doc.rs" "/checkout/library/proc_macro/src/bridge/buffer.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.

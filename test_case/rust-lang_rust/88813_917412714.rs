diff
diff --git a/library/backtrace b/library/backtrace
--- a/library/backtrace
+++ b/library/backtrace
@@ -1 +1 @@
-Subproject commit 4f925f8d81dfa57067537217e501e1dff7433491
+Subproject commit 4f925f8d81dfa57067537217e501e1dff7433491-dirty
diff --git a/src/bootstrap/doc.rs b/src/bootstrap/doc.rs
index c8714117930..b12bd43ab29 100644
--- a/src/bootstrap/doc.rs
+++ b/src/bootstrap/doc.rs
@@ -590,10 +590,16 @@ fn run(self, builder: &Builder<'_>) {
         cargo.rustdocflag("-Znormalize-docs");
         cargo.rustdocflag("--show-type-layout");
@@ -590,10 +590,15 @@ fn run(self, builder: &Builder<'_>) {
         cargo.rustdocflag("-Znormalize-docs");
         cargo.rustdocflag("--show-type-layout");
         compile::rustc_cargo(builder, &mut cargo, target);
+
+        cargo.arg("-Zunstable-options");
         cargo.arg("-Zskip-rustdoc-fingerprint");
 
         // Only include compiler crates, no dependencies of those, such as `libc`.
+        // Do link to dependencies on `docs.rs` however using `rustdoc-map`.
         cargo.arg("--no-deps");
+        cargo.arg("-Zrustdoc-map");
+        cargo.arg(r#"--config=doc.extern-map.registries.crates-io="https://docs.rs""#);
 
         // Find dependencies for top level crates.
         let mut compiler_crates = HashSet::new();


diff --git a/src/librustpkg/package_source.rs b/src/librustpkg/package_source.rs
index c2fddaf..00a764b 100644
--- a/src/librustpkg/package_source.rs
+++ b/src/librustpkg/package_source.rs
@@ -170,7 +170,7 @@ impl PkgSrc {
         // We use a temporary directory because if the git clone fails,
         // it creates the target directory anyway and doesn't delete it

-        let scratch_dir = extra::tempfile::mkdtemp(&os::tmpdir(), "rustpkg");
+        let scratch_dir = extra::tempfile::mkdtemp(&Path(&"/home/dpc/lab/rust/tmp"), "rustpkg");
         let clone_target = match scratch_dir {
             Some(d) => d.push("rustpkg_temp"),
             None    => cond.raise(~"Failed to create temporary directory for fetching git sources")

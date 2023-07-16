diff
diff --git a/tirocks-sys/build.rs b/tirocks-sys/build.rs
index 1e1bcf7..b65f479 100644
--- a/tirocks-sys/build.rs
+++ b/tirocks-sys/build.rs
@@ -206,7 +206,7 @@ fn build_titan(build: &mut Build) {
     build.include(cur_dir.join("titan"));
 }

-fn build_rocksdb(build: &mut Build) {
+fn build_rocksdb(build: &mut Build) -> PathBuf {
     let target = env::var("TARGET").expect("TARGET was not set");
     let mut cfg = Config::new("rocksdb");
     cfg.out_dir(format!("{}/rocksdb", env::var("OUT_DIR").unwrap()));
@@ -225,7 +225,6 @@ fn build_rocksdb(build: &mut Build) {
         .build_target("rocksdb")
         .very_verbose(true)
         .build();
-    figure_link_lib(&dst, "rocksdb");

     if cfg!(target_os = "windows") {
         build.define("OS_WIN", None);
@@ -251,6 +250,11 @@ fn build_rocksdb(build: &mut Build) {
         build.define("OPENSSL", None);
     }

+    dst
+}
+
+fn link_rocksdb(dst: PathBuf) {
+    figure_link_lib(&dst, "rocksdb");
     println!("cargo:rustc-link-lib=static=z");
     println!("cargo:rustc-link-lib=static=bz2");
     println!("cargo:rustc-link-lib=static=lz4");
@@ -262,13 +266,15 @@ fn main() {
     patch_libz_env();
     let mut build = Build::new();
     build_titan(&mut build);
-    build_rocksdb(&mut build);
+    let rocksdb_dst = build_rocksdb(&mut build);

     build.cpp(true).file("crocksdb/c.cc");
     if !cfg!(target_os = "windows") {
         build.flag("-std=c++11");
         build.flag("-fno-rtti");
     }
-    link_cpp(&mut build);
     build.warnings(false).compile("libcrocksdb.a");
+
+    link_rocksdb(rocksdb_dst);
+    link_cpp(&mut build);
 }

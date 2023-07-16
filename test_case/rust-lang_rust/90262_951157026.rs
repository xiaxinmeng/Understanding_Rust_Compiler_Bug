diff
--- a/src/cargo/core/source/source_id.rs
+++ b/src/cargo/core/source/source_id.rs
@@ -621,7 +621,13 @@
 fn test_cratesio_hash() {
     let config = Config::default().unwrap();
     let crates_io = SourceId::crates_io(&config).unwrap();
-    assert_eq!(crate::util::hex::short_hash(&crates_io), "1ecc6299db9ec823");
+    assert!([
+        "1ecc6299db9ec823", // 64 LE
+        "1285ae84e5963aae", // 32 LE
+        "eae4ba8cbf2ce1c7", // 64 BE
+        "b420f105fcaca6de", // 32 BE
+    ]
+    .contains(&crate::util::hex::short_hash(&crates_io).as_str()));
 }
 
 /// A `Display`able view into a `SourceId` that will write it as a url

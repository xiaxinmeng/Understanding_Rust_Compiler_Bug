diff
diff --git a/library/std/src/path.rs b/library/std/src/path.rs
index 8ecea8ce07f..c03d197e019 100644
--- a/library/std/src/path.rs
+++ b/library/std/src/path.rs
@@ -168,8 +168,8 @@ pub enum Prefix<'a> {
 
     /// Device namespace prefix, e.g., `\\.\COM42`.
     ///
-    /// Device namespace prefixes consist of `\\.\` immediately followed by the
-    /// device name.
+    /// Device namespace prefixes consist of `\\.\` (possibly using `/`
+    /// instead of `\`), immediately followed by the device name.
     #[stable(feature = "rust1", since = "1.0.0")]
     DeviceNS(#[stable(feature = "rust1", since = "1.0.0")] &'a OsStr),
 
diff --git a/library/std/src/path/tests.rs b/library/std/src/path/tests.rs
index d1f59d2786e..0d8ea29c2be 100644
--- a/library/std/src/path/tests.rs
+++ b/library/std/src/path/tests.rs
@@ -971,15 +971,15 @@ pub fn test_decompositions_windows() {
     file_prefix: None
     );
 
-    t!("\\\\?\\C:/foo",
-    iter: ["\\\\?\\C:/foo"],
+    t!("\\\\?\\C:/foo/bar",
+    iter: ["\\\\?\\C:", "\\", "foo/bar"],
     has_root: true,
     is_absolute: true,
-    parent: None,
-    file_name: None,
-    file_stem: None,
+    parent: Some("\\\\?\\C:/"),
+    file_name: Some("foo/bar"),
+    file_stem: Some("foo/bar"),
     extension: None,
-    file_prefix: None
+    file_prefix: Some("foo/bar")
     );
 
     t!("\\\\.\\foo\\bar",
diff --git a/library/std/src/sys/windows/path.rs b/library/std/src/sys/windows/path.rs
index 7bf6f4a9b1f..cbe6d1630e1 100644
--- a/library/std/src/sys/windows/path.rs
+++ b/library/std/src/sys/windows/path.rs
@@ -127,14 +127,14 @@ pub fn parse_prefix(path: &OsStr) -> Option<Prefix<'_>> {
                 Some(VerbatimUNC(server, share))
             } else {
                 let path = parser.finish();
-                let (prefix, _) = parse_next_component(path, true);
 
                 // in verbatim paths only recognize an exact drive prefix
-                if let Some(drive) = parse_drive_exact(prefix) {
+                if let Some(drive) = parse_drive_exact(path) {
                     // \\?\C:
                     Some(VerbatimDisk(drive))
                 } else {
                     // \\?\prefix
+                    let (prefix, _) = parse_next_component(path, true);
                     Some(Verbatim(prefix))
                 }
             }
@@ -166,24 +166,24 @@ pub fn parse_prefix(path: &OsStr) -> Option<Prefix<'_>> {
 }
 
 // Parses a drive prefix, e.g. "C:" and "C:\whatever"
-fn parse_drive(prefix: &OsStr) -> Option<u8> {
+fn parse_drive(path: &OsStr) -> Option<u8> {
     // In most DOS systems, it is not possible to have more than 26 drive letters.
     // See <https://en.wikipedia.org/wiki/Drive_letter_assignment#Common_assignments>.
     fn is_valid_drive_letter(drive: &u8) -> bool {
         drive.is_ascii_alphabetic()
     }
 
-    match prefix.bytes() {
+    match path.bytes() {
         [drive, b':', ..] if is_valid_drive_letter(drive) => Some(drive.to_ascii_uppercase()),
         _ => None,
     }
 }
 
 // Parses a drive prefix exactly, e.g. "C:"
-fn parse_drive_exact(prefix: &OsStr) -> Option<u8> {
+fn parse_drive_exact(path: &OsStr) -> Option<u8> {
     // only parse two bytes: the drive letter and the drive separator
-    if prefix.len() == 2 { parse_drive(prefix) } else { None }
+    if path.bytes().get(2).map(|&x| is_sep_byte(x)).unwrap_or(true) {
+        parse_drive(path)
+    } else {
+        None
+    }
 }
 
 // Parse the next path component.
diff --git a/library/std/src/sys/windows/path/tests.rs b/library/std/src/sys/windows/path/tests.rs
index 0bf5bbe4883..8656b04e4f4 100644
--- a/library/std/src/sys/windows/path/tests.rs
+++ b/library/std/src/sys/windows/path/tests.rs
@@ -108,7 +108,7 @@ fn test_parse_prefix_verbatim() {
 
 #[test]
 fn test_parse_prefix_verbatim_device() {
-    let prefix = Some(Prefix::DeviceNS(OsStr::new("?")));
+    let prefix = Some(Prefix::UNC(OsStr::new("?"), OsStr::new("C:")));
     assert_eq!(prefix, parse_prefix(r"//?/C:/windows/system32/notepad.exe"));
     assert_eq!(prefix, parse_prefix(r"//?/C:\windows\system32\notepad.exe"));
     assert_eq!(prefix, parse_prefix(r"/\?\C:\windows\system32\notepad.exe"));

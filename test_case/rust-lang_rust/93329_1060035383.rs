diff
diff --git a/src/bootstrap/bin/rustc.rs b/src/bootstrap/bin/rustc.rs
index 7105a2457e2..bdfce66c2d6 100644
--- a/src/bootstrap/bin/rustc.rs
+++ b/src/bootstrap/bin/rustc.rs
@@ -175,6 +175,19 @@ fn main() {
     let (child, status) = {
         let errmsg = format!("\nFailed to run:\n{:?}\n-------------", cmd);
         let mut child = cmd.spawn().expect(&errmsg);
+        let child_id = child.id().to_string();
+        std::thread::spawn(move || {
+            std::thread::sleep(std::time::Duration::from_secs(1000));
+            let status =
+                Command::new("C:\\Program Files (x86)\\Windows Kits\\10\\Debuggers\\x64\\cdb.exe")
+                    .args(["-c", "~*k;qd", "-p"])
+                    .arg(&child_id)
+                    .status()
+                    .unwrap();
+            if !status.success() {
+                eprintln!("cdb err?");
+            }
+        });
         let status = child.wait().expect(&errmsg);
         (child, status)
     };

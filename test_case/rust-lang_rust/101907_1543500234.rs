diff
diff --git a/src/bootstrap/download.rs b/src/bootstrap/download.rs
index 3e82a381a1b..7600365d29b 100644
--- a/src/bootstrap/download.rs
+++ b/src/bootstrap/download.rs
@@ -568,6 +568,7 @@ pub(crate) fn maybe_download_ci_llvm(&self) {
         let llvm_root = self.ci_llvm_root();
         let llvm_stamp = llvm_root.join(".llvm-stamp");
         let llvm_sha = detect_llvm_sha(&self, self.rust_info.is_managed_git_subrepository());
+        let llvm_sha = "PUT_SHA_HERE";
         let key = format!("{}{}", llvm_sha, self.llvm_assertions);
         if program_out_of_date(&llvm_stamp, &key) && !self.dry_run() {
             self.download_ci_llvm(&llvm_sha);

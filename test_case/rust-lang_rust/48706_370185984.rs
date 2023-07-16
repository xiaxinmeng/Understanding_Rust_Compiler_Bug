diff
--- a/src/librustc/middle/entry.rs
+++ b/src/librustc/middle/entry.rs
@@ -178,6 +178,9 @@ fn configure_main(this: &mut EntryContext, crate_name: &str) {
             err.emit();
             this.session.abort_if_errors();
         } else {
+            if let Some(ref filename) = this.session.local_crate_source_file {
+                err.note(&format!("consider adding a main function to {}", filename.display()));
+            }
             if this.session.teach(&err.get_code().unwrap()) {
                 err.note("If you don't know the basics of Rust, you can go look to the Rust Book \
                           to get started: https://doc.rust-lang.org/book/");

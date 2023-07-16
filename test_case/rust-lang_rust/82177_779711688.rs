plain
   Compiling toml v0.5.7
error: unused doc comment
  --> src/bootstrap/clean.rs:24:9
   |
24 | /         /// Only delete the bootstrap executable on non-Windows systems 
25 | |         /// Windows does not allow deleting a currently running executable
   | |__________________________________________________________________________^
26 |           #[cfg(not(windows))]
27 |           rm_rf(&build.out.join("bootstrap"));
   |           ----------------------------------- rustdoc does not generate documentation for expressions
   |
   = note: `-D unused-doc-comments` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `bootstrap`


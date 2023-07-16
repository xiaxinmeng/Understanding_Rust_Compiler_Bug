
error: Could not compile `bug`.

Caused by:
  Process didn't exit successfully: `rustc src/main.rs --crate-name bug --crate-type bin -g --out-dir /path/to/bug/target/debug --emit=dep-info,link -L dependency=/path/to/bug/target/debug -L dependency=/path/to/bug/target/debug/deps --extern bug=/path/to/bug/target/debug/libbug.rlib` (signal: 11, SIGSEGV: invalid memory reference)

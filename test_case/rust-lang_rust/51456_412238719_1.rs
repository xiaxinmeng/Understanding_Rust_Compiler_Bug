\n\nIf the item you are importing is not defined in some super-module of the\ncurrent module, then it must also be declared as public (e.g., `pub fn`).\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/crate-in-paths.rs","byte_start":570,"byte_end":573,"line_start":20,"line_end":20,"column_start":5,"column_end":8,"is_primary":true,"text":[{"text":"    Foo;","highlight_start":5,"highlight_end":8}],"label":"not found in this scope","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0425]: cannot find value `Foo` in this scope\n  --> /checkout/src/test/ui/crate-in-paths.rs:20:5\n   |\nLL |     Foo;\n   |     ^^^ not found in this scope\n\n"}
[00:49:33] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:49:33] {"message":"For more information about this error, try `rustc --explain E0425`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0425`.\n"}
[00:49:33] ------------------------------------------
[00:49:33] 
[00:49:33] thread '[ui] ui/crate-in-paths.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:49:33] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:49:33] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:49:33] 
[00:49:33] ---- [ui] ui/hygiene/globs.rs stdout ----
[00:49:33] diff of stderr:
[00:49:33] 
[00:49:33] 3    |
[00:49:33] 4 LL |         f(ned in the current module, it must be imported using a\n`use` statement, like so:\n\n
\n"},"level":"error","spans":[{"file_name":"/c E0432`.\n"}
[00:53:40] ------------------------------------------
[00:53:40] 
[00:53:40] thread '[ui] ui/imports/issue-56125.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:53:40] 
[00:53:40] 
[00:53:40] ---- [ui] ui/imports/shadow_builtin_macros.rs stdout ----
[00:53:40] diff of stderr:
[00:53:40] 
[00:53:40] 4 LL |     fn f() { panic!(); } //~ ERROR ambiguous
[00:53:40] 6    |
[00:53:40] 6    |
[00:53:40] -    = note: `panic` could refer to a macro from prelude
[00:53:40] + note: `panic` could refer to a macro from prelude
[00:53:40] 8 note: `panic` could also refer to the macro imported here
[00:53:40] 10    |
[00:53:40] 
[00:53:40] 
[00:53:40] 19 LL |     fn f() { panic!(); } //~ ERROR ambiguous
[00:53:40] 21    |
[00:53:40] 21    |
[00:53:40] -    = note: `panic` could refer to a macro from prelude
[00:53:40] + note: `panic` could refer to a macro from prelude
[00:53:40] 23 note: `panic` could also refer to the macro imported here
[00:53:40] 25    |
[00:53:40] 
[00:53:40] 
[00:53:40] 33 LL |     panic!(); //~ ERROR `panic` is ambiguous
[00:53:40] 35    |
[00:53:40] 35    |
[00:53:40] -    = note: `panic` could refer to a macro from prelude
[00:53:40] + note: `panic` could refer to a macro from prelude
[00:53:40] 37 note: `panic` could also refer to the macro defined here
[00:53:40] 39    |
[00:53:40] 
[00:53:40] 
[00:53:40] 
[00:53:h::*;\n}\n\nfn main() {\n    collider::foo(); // ERROR: `foo` is ambiguous\n}\n
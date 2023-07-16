compile_fail\n#![feature(box_sysrc/test/ui/borrowck/two-phase-nonrecv-autoref.rs:173:7\n   |\nLL |     i[i[3]] = i[4];\n   |     --^----\n   |     | |\n   |     | immutable borrow occurs here\n   |     mutable borrow occurs here\n   |     mutable borrow later used here\n\n"}
[00:47:58] {"message":"aborting due to 9 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 9 previous errors\n\n"}
[00:47:58] {"message":"Some errors occurred: E0161, E0382, E0499, E0502.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0161, E0382, E0499, E0502.\n"}
[00:47:58] {"message":"For more information about an error, try `rustc --explain E0161`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0161`.\n"}
[00:47:58] ------------------------------------------
[00:47:58] 
[00:47:58] thread '[ui] ui/borrowck/two-phase-nonrecv-autoref.rs#nll' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:47:58] 
[00:47:58] 
[00:47:58] ---- [ui] ui/issues/issue-29723.rs stdout ----
[00:47:58] diff of stderr:
[00:47:58] 
[00:47:58] 5    |                     - value moved here
[00:47:58] 6 ...
[00:47:58] 7 LL |             s
[00:47:58] -    |             ^ value used here after move
[00:47:58] +    |             ^ value used here after partial move
[00:47:58] 9    |
[00:47:58] 10    = note: move occurs because `s` has type `std::string::String`, which does not implement the `Copy` trait
[00:47:58] 
[00:47:58] 
[00:47:58] The actual stderr differed from the expected stderr.
58] stdout:
58] stdout:
[00:47:58] ------------------------------------------
[00:47:58] 
[00:47:58] ------------------------------------------
[00:47:58] stderr:
[00:47:58] ------------------------------------------
[00:47:58] {"message":"use of moved value: `x` (Ast)","code":{"code":"E0382","explanation":"\nThis error occurs when an attempt is made to use a variable after its contents\nhave been moved elsewhere. For example:\n\n
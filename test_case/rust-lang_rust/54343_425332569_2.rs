\n#[derive(Copy, Clone)]\nstruct Point { x: i32, y: i32 }\n\nfn main() {\n    let mut p1 = Point{ x: -1, y: 2 };\n 53] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:53] ---- [ui] ui/borrowck/borrowck-drop-from-guard.rs stdout ----
[00:47:53] diff of stderr:
[00:47:53] 
[00:47:53] 2   --> $DIR/borrowck-drop-from-guard.rs:22:23
[00:47:53] 2   --> $DIR/borrowck-drop-from-guard.rs:22:23
[00:47:53] 3    |
[00:47:53] 4 LL |         Some(_) if { drop(my_str); false } => {}
[00:47:53] -    |                           ------ value moved here
[00:47:53] +    |                           ------ value moved here 
[00:47:53] 6 LL |         Some(_) => {}
[00:47:53] 7 LL |         None => { foo(my_str); } //~ ERROR [E0382]
[00:47:53] 8    |                       ^^^^^^ value used here after move
[00:47:53] 
[00:47:53] The actual stderr differed from the expected stderr.
[00:47:53] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-drop-from-guard/borrowck-drop-from-guard.stderr
[00:47:53] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-drop-from-guard/borrowck-drop-from-guard.stderr
[00:47:53] To update references, rerun the tests and pass the `--bless` flag
[00:47:53] To only update this specific test, also pass `--test-args borrowck/borrowck-drop-from-guard.rs`
[00:47:53] error: 1 errors occurred comparing output.
[00:47:53] status: exit code: 1
[00:47:53] status: exit code: 1
[00:47:53] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-drop-from-guard.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-drop-from-guard/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-drop-from-guard/auxiliary" "-A" "unused"
[00:47:53] ------------------------------------------
[00:47:53] 
[00:47:53] ------------------------------------------
[00:47:53] stderr:
[00:47:53] stderr:
[00:47:53] ------------------------------------------
[00:47:53] {"message":"use of moved value: `my_str`","code":{"code":"E0382","explanation":"\nThis error occurs when an attempt is made to use a variable after its contents\nhave been moved elsewhere. For example:\n\n
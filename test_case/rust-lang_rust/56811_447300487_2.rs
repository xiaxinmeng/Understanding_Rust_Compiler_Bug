\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/borrowck-asm.rs","byte_start":1452,"byte_end":1453,"line_start":52,"line_end":52,"column_start":13,"column_end":14,"is_primary":false,"text":[{"text":"        let x = 3;","highlight_start":13,"highlight_end":14}],"labep-from-guard.rs stdout ----
[00:47:58] 
[00:47:58] 5    |                           ------ value moved here
[00:47:58] 5    |                           ------ value moved here
[00:47:58] 6 LL |         Some(_) => {}
[00:47:58] 7 LL |         None => { foo(my_str); } //~ ERROR [E0382]
[00:47:58] -    |                       ^^^^^^ value used here after move
[00:47:58] +    |                       ^^^^^^ value used here after partial move
[00:47:58] 9    |
[00:47:58] 10    = note: move occurs because `my_str` has type `std::string::String`, which does not implement the `Copy` trait
[00:47:58] 
[00:47:58] 
[00:47:58] The actual stderr differed from the expected stderr.
[00:47:58] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-drop-from-guard/borrowck-drop-from-guard.stderr
[00:47:58] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-drop-from-guard/borrowck-drop-from-guard.stderr
[00:47:58] To update references, rerun the tests and pass the `--bless` flag
[00:47:58] To only update this specific test, also pass `--test-args borrowck/borrowck-drop-from-guard.rs`
[00:47:58] error: 1 errors occurred comparing output.
[00:47:58] status: exit code: 1
[00:47:58] status: exit code: 1
[00:47:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-drop-from-guard.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-drop-from-guard/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrow] 
[00:47:58] The actual stderr differed from the expected stderr.
[00:47:58] The actual stderr differed from the expected stderr.
[00:47:58] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-describe-lvalue.mir/borrowck-describe-lvalue.mir.stderr
[00:47:58] To update references, rerun the tests and pass the `--bless` flag
[00:47:58] To only update this specific test, also pass `--test-args borrowck/borrowck-describe-lvalue.rs`
[00:47:58] 
[00:47:58] error in revision `mir`: 1 errors occurred comparing output.
[00:47:58] status: exit code: 1
[00:47:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-describe-lvalue.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "mir" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-describe-lvalue.mir/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-describe-lvalue.mir/auxiliary" "-A" "unused"
[00:47:58] ------------------------------------------
[00:47:58] 
[00:47:58] ------------------------------------------
[00:47:58] stderr:
[00:47:58] stderr:
[00:47:58] ------------------------------------------
[00:47:58] {"message":"cannot borrow `x` as mutable more than once at a time","code":{"code":"E0499","explanation":"\nA variable was borrowed as mutable more than once. Erroneous code example:\n\n
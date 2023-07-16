\n\nFor more information on the rust ownership system, take a look at\nhttps://doc.rust-lang.org/stable/book/references-and-borrowing.html.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/two-phase-nonrecv-autoref.rs","byte_start":6021,"byte_end":6022,"line_start":173,"line_end":173,"column_start":7,"column_end":8,"is_primary":true,"text":[{"text":"    i[i[3]] = i[4];","highlight_start":7,"highlight_end":8}],"label":"immutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/two-phase-nonrecv-autoref.rs","byte_start":6019,"byte_end":6020,"line_start":173,"line_end":173,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    i[i[3]] = i[4];","highlight_start":5,"highlight_end":6}],"label":"mutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/two-phase-nonrecv-autoref.rs","byte_start":6019,"byte_end":6026,"line_start":173,"line_end":173,"column_start":5,"column_end":12,"is_primary":false,"text":[{"text":"    i[i[3]] = i[4];","highlight_start":5,"highlight_end":12}],"label":"borrow later used here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0502]: cannot borrow `i` as immutable because it is also borrowed as mutable\n  --> /checkout/src/test/ui/borrowck/two-phase-nonrecv-autoref.rs:173:7\n   |\nLL |     i[i[3]] = i[4];\n   |     --^----\n   |     | |\n   |     | immutable borrow occurs here\n   |     mutable borrow occurs here\n   |     borrow later used here\n\n"}
[00:48:10] {"message":"aborting due to 9 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 9 previous errors\n\n"}
[00:48:10] {"message":"Some errors occurred: E0161, E0382, E0499, E0502.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0161, E0382, E0499, E0502.\n"}
[00:48:10] {"message":"For more information about an error, try `rustc --explain E0161`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0161`.\n"}
[00:48:10] ------------------------------------------
[00:48:10] 
[00:48:10] thread '[ui] ui/borrowck/two-phase-nonrecv-autoref.rs#nll' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3258:9
[00:48:10] 
[00:48:10] 
[00:48:10] ---- [ui] ui/borrowck/two-phase-surprise-no-conflict.rs#nll stdout ----
[00:48:10] 
[00:48:10] error in revision `nll`: Error: expected failure status (Some(1)) but received status Some(101).
[00:48:10] status: exit code: 101
[00:48:10] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/two-phase-surprise-no-conflict.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "nll" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-surprise-no-conflict.nll/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-Z" "two-phase-borrows" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-surprise-no-conflict.nll/auxiliary" "-A" "unused"
[00:48:10] ------------------------------------------
[00:48:10] 
[00:48:10] ------------------------------------------
[00:48:10] stderr:
[00:48:10] stderr:
[00:48:10] ------------------------------------------
[00:48:10] {"message":"nfn main() {\n    let mut value = 3;\n    // By creating a new block, you can limit the scope\n    // of the reference.\n    {\n        let _borrow = &mut value; // Use `_borrow` inside this block.\n    }\n    // The block has ended and with it the borrow.\n    // You can now use `value` again.\n    let _sum = value + 1;\n}\n
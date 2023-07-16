\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-45697.rs","byte_start":936,"byte_end":937,"line_start":29,"line_end":29,"column_start":40,"column_end":41,"is_primary":false,"text":[{"text":"        let z = copy_borrowed_ptr(&mut y);","highlight_start":40,"highlight_end":41}],"label":"borrow of `*y.pointer` occurs here","suggested_replacement":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issue-45697.rs","byte_start":948,"byte_end":963,"line_start":30,"line_end":30,"column_start":9,"column_end":24,"is_primary":true,"text":[{"text":"        *y.pointer += 1;","highlight_start":9,"highlight_end":24}],"label":"assignment to borrowed `*y.pointer` occurs here","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error[E0506]: cannot assign to `*y.pointer` because it is borrowed (Ast)\n  --> /checkout/src/test/ui/issue-45697.rs:30:9\n   |\nLL |         let z = copy_borrowed_ptr(&mut y);\n   |                                        - borrow of `*y.pointer` occurs here\nLL |         *y.pointer += 1;\n   |         ^^^^^^^^^^^^^^^ assignment to borrowed `*y.pointer` occurs here\n\n"}
[00:42:43] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:42:43] {"message":"Some errors occurred: E0503, E0506.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0503, E0506.\n"}
[00:42:43] {"message":"For more information about an error, try `rustc --explain E0503`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0503`.\n"}
---
[00:42:43] - error[E0597]: `z` does not live long enough (Ast)
[00:42:43] -   --> $DIR/issue-46471-1.rs:16:14
[00:42:43] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:478:22
[00:42:43] + error[E0597]: `z` does not live long enough (Mir)
[00:42:43] +   --> $DIR/issue-46471-1.rs:16:9
[00:42:43] 3    |
[00:42:43] 4 LL |         &mut z
[00:42:43] -    |              ^ borrowed value does not live long enough
[00:42:43] +    |         ^^^^^^ borrowed value does not live long enough
[00:42:43] 6 LL |     };
[00:42:43] 7    |     - `z` dropped here while still borrowed
[00:42:43] 8 ...
[00:42:43]
[00:42:43] 9 LL | }
[00:42:43] 10    | - borrowed value needs to live until here
[00:42:43] 11
[00:42:43] - error[E0597]: `z` does not live long enough (Mir)
[00:42:43] -   --> $DIR/issue-46471-1.rs:16:9
[00:42:43] + error[E0597]: `z` does not live long enough (Ast)
[00:42:43] +   --> $DIR/issue-46471-1.rs:16:14
[00:42:43] 14    |
[00:42:43] 15 LL |         &mut z
[00:42:43] -    |         ^^^^^^ borrowed value does not live long enough
[00:42:43] +    |              ^ borrowed value does not live long enough
[00:42:43] 17 LL |     };
[00:42:43] 18    |     - `z` dropped here while still borrowed
[00:42:43] 19 ...
[00:42:43]
[00:42:43]
[00:42:43] The actual stderr differed from the expected stderr.
[00:42:43] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-46471-1.stderr
[00:42:43] To update references, run this command from build directory:
[00:42:43] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'issue-46471-1.rs'
[00:42:43]
[00:42:43] error: 1 errors occurred comparing output.
[00:42:43] status: exit code: 101
[00:42:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-46471-1.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-46471-1.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "emit-end-regions" "-Z" "borrowck=compare" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-46471-1.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:42:43] {"message":"`z` does not live long enough (Mir)","code":{"code":"E0597","explanation":"\nThis error occurs because a borrow was made inside a variable which has a\ngreater lifetime than the borrowed one.\n\nExample of erroneous code:\n\n
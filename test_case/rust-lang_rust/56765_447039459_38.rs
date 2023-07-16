\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/associated-types/associated-types-path-2.rs","byte_start":754,"byte_end":758,"line_start":29,"line_end":29,"column_start":14,"column_end":18,"is_primary":true,"text":[{"text":"    f1(2i32, 4i32);","highlight_start":14,"highlight_end":18}],"label":"expected u32, found i32","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0308]: mismatched types\n  --> /checkout/src/test/ui/associated-types/associated-types-path-2.rs:29:14\n   |\nLL |     f1(2i32, 4i32);\n   |              ^^^^ expected u32, found i32\n\n"}
[00:47:45] {"message":"aborting due to 6 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 6 previous errors\n\n"}
[00:47:45] {"message":"Some errors occurred: E0277, E0308.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0277, E0308.\n"}
[00:47:45] {"message":"For more information about an error, try `rustc --explain E0277`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0277`.\n"}
[00:47:45] ------------------------------------------
[00:47:45] 
[00:47:45] thread '[ui] ui/associated-types/associated-types-path-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:47:45] 
[00:47:45] 
[00:47:45] ---- [ui] ui/binop/binop-consume-args.rs stdout ----
[00:47:45] diff of stderr:
[00:47:45] 
[00:47:45] 1 error[E0382]: use of moved value: `lhs`
[00:47:45] +   --> $DIR/binop-consume-args.rs:47:10
[00:47:45] 3    |
[00:47:45] 3    |
[00:47:45] - LL |     lhs + rhs;
[00:47:45] + LL |     lhs & rhs;
[00:47:45] 5    |     --- value moved here
[00:47:45] 6 LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
[00:47:45] 7    |          ^^^ value used here after move
[00:47:45] 
[00:47:45] 9    = note: move occurs because `lhs` has type `A`, which does not implement the `Copy` trait
[00:47:45] 10 
[00:47:45] 11 error[E0382]: use of moved value: `rhs`
[00:47:45] +   --> $DIR/binop-consume-args.rs:48:10
[00:47:45] 13    |
[00:47:45] 13    |
[00:47:45] - LL |     lhs + rhs;
[00:47:45] + LL |     lhs & rhs;
[00:47:45] 15    |           --- value moved here
[00:47:45] 16 LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
[00:47:45] 17 LL |     drop(rhs);  //~ ERROR use of moved value: `rhs`
[00:47:45] 
[00:47:45] 20    = note: move occurs because `rhs` has type `B`, which does not implement the `Copy` trait
[00:47:45] 21 
[00:47:45] 22 error[E0382]: use of moved value: `lhs`
[00:47:45] +   --> $DIR/binop-consume-args.rs:53:10
[00:47:45] 24    |
[00:47:45] 24    |
[00:47:45] - LL |     lhs - rhs;
[00:47:45] + LL |     lhs | rhs;
[00:47:45] 26    |     --- value moved here
[00:47:45] 27 LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
[00:47:45] 28    |          ^^^ value used here after move
[00:47:45] 
[00:47:45] 30    = note: move occurs because `lhs` has type `A`, which does not implement the `Copy` trait
[00:47:45] 31 
[00:47:45] 32 error[E0382]: use of moved value: `rhs`
[00:47:45] +   --> $DIR/binop-consume-args.rs:54:10
[00:47:45] 34    |
[00:47:45] 34    |
[00:47:45] - LL |     lhs - rhs;
[00:47:45] + LL |     lhs | rhs;
[00:47:45] 36    |           --- value moved here
[00:47:45] +   --> $DIR/binop-consume-args.rs:72:10
[00:47:45] 97    |
[00:47:45] 97    |
[00:47:45] - LL |     lhs % rhs;
[00:47:45] -    |           --- value moved here
[00:47:45] + LL |     lhs >> rhs;
[00:47:45] +    |            --- value moved here
[00:47:45] 100 LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
[00:47:45] 101 LL |     drop(rhs);  //~ ERROR use of moved value: `rhs`
[00:47:45] 102    |          ^^^ value used here after move
[00:47:45] 
[00:47:45] 104    = note: move occurs because `rhs` has type `B`, which does not implement the `Copy` trait
[00:47:45] 105 
[00:47:45] 106 error[E0382]: use of moved value: `lhs`
[00:47:45] +   --> $DIR/binop-consume-args.rs:29:10
[00:47:45] 108    |
[00:47:45] 108    |
[00:47:45] - LL |     lhs & rhs;
[00:47:45] + LL |     lhs * rhs;
[00:47:45] 110    |     --- value moved here
[00:47:45] 111 LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
[00:47:45] 112    |          ^^^ value used here after move
[00:47:45] 
[00:47:45] 114    = note: move occurs because `lhs` has type `A`, which does not implement the `Copy` trait
[00:47:45] 115 
[00:47:45] 116 error[E0382]: use of moved value: `rhs`
[00:47:45] +   --> $DIR/binop-consume-args.rs:30:10
[00:47:45] 118    |
[00:47:45] 118    |
[00:47:45] - LL |     lhs & rhs;
[00:47:45] + LL |     lhs * rhs;
[00:47:45] 120    |           --- value moved here
[00:47:45] 121 LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
[00:47:45] 122 LL |     drop(rhs);  //~ ERROR use of moved value: `rhs`
[00:47:45] 
[00:47:45] 125    = note: move occurs because `rhs` has type `B`, which does not implement the `Copy` trait
[00:47:45] 126 
[00:47:45] 127 error[E0382]: use of moved value: `lhs`
[00:47:45] +   --> $DIR/binop-consume-args.rs:35:10
[00:47:45] 129    |
[00:47:45] 129    |
[00:47:45] - LL |     lhs | rhs;
[00:47:45] + LL |     lhs / rhs;
[00:47:45] 131    |     --- value moved here
[00:47:45] 132 LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
[00:47:45] 133    |          ^^^ value used here after move
[00:47:45] 
[00:47:45] 135    = note: move occurs because `lhs` has type `A`, which does not implement the `Copy` trait
[00:47:45] 136 
[00:47:45] 137 error[E0382]: use of moved value: `rhs`
[00:47:45] +   --> $DIR/binop-consume-args.rs:36:10
[00:47:45] 139    |
[00:47:45] 139    |
[00:47:45] - LL |     lhs | rhs;
[00:47:45] + LL |     lhs / rhs;
[00:47:45] 141    |           --- value moved here
[00:47:45] 142 LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
[00:47:45] 143 LL |     drop(rhs);  //~ ERROR use of moved value: `rhs`
[00:47:45] 
[00:47:45] 146    = note: move occurs because `rhs` has type `B`, which does not implement the `Copy` trait
[00:47:45] 147 
[00:47:45] 148 error[E0382]: use of moved value: `lhs`
[00:47:45] +   --> $DIR/binop-consume-args.rs:41:10
[00:47:45] 150    |
[00:47:45] 150    |
[00:47:45] - LL |     lhs ^ rhs;
[00:47:45] + LL |     lhs % rhs;
[00:47:45] 152    |     --- value moved here
[00:47:45] 153 LL |     drop(lhs);  //~ ERROR use of moved + LL |     lhs - rhs;
[00:47:45] +    |           --- value moved here
[00:47:45] 184 LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
[00:47:45] 185 LL |     drop(rhs);  //~ ERROR use of moved value: `rhs`
[00:47:45] 186    |          ^^^ value used here after move
[00:47:45] 
[00:47:45] 188    = note: move occurs because `rhs` has type `B`, which does not implement the `Copy` trait
[00:47:45] 189 
[00:47:45] 190 error[E0382]: use of moved value: `lhs`
[00:47:45] +   --> $DIR/binop-consume-args.rs:17:10
[00:47:45] 192    |
[00:47:45] 192    |
[00:47:45] - LL |     lhs >> rhs;
[00:47:45] + LL |     lhs + rhs;
[00:47:45] 194    |     --- value moved here
[00:47:45] 195 LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
[00:47:45] 196    |          ^^^ value used here after move
[00:47:45] 
[00:47:45] 198    = note: move occurs because `lhs` has type `A`, which does not implement the `Copy` trait
[00:47:45] 199 
[00:47:45] 200 error[E0382]: use of moved value: `rhs`
[00:47:45] +   --> $DIR/binop-consume-args.rs:18:10
[00:47:45] 202    |
[00:47:45] 202    |
[00:47:45] - LL |     lhs >> rhs;
[00:47:45] -    |            --- value moved here
[00:47:45] + LL |     lhs + rhs;
[00:47:45] +    |           --- value moved here
[00:47:45] 205 LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
[00:47:45] 206 LL |     drop(rhs);  //~ ERROR use of moved value: `rhs`
[00:47:45] 207    |          ^^^ value used here after move
[00:47:45] 
[00:47:45] The actual stderr differed from the expected stderr.
[00:47:45] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-consume-args/binop-consume-args.stderr
[00:47:45] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-consume-args/binop-consume-args.stderr
[00:47:45] To update references, rerun the tests and pass the `--bless` flag
[00:47:45] To only update this specific test, also pass `--test-args binop/binop-consume-args.rs`
[00:47:45] error: 1 errors occurred comparing output.
[00:47:45] status: exit code: 1
[00:47:45] status: exit code: 1
[00:47:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binop/binop-consume-args.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-consume-args/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-consume-args/auxiliary" "-A" "unused"
[00:47:45] ------------------------------------------
[00:47:45] 
[00:47:45] ------------------------------------------
[00:47:45] stderr:
[00:47:45] stderr:
[00:47:45] ------------------------------------------
[00:47:45] {"message":"use of moved value: `lhs`","code":{"code":"E0382","explanation":"\nThis error occurs when an attempt is made to use a variable after its contents\nhave been moved elsewhere. For example:\n\n
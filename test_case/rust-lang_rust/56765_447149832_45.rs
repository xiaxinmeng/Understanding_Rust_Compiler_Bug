\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/associated-types/associated-types-path-2.rs","byte_start":754,"byte_end":758,"line_start":29,"line_end":29,"column_start":14,"column_end":18,"is_primary":true,"text":[{"text":"    f1(2i32, 4i32);","highlight_start":14,"highlight_end":18}],"label":"expected u32, found i32","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0308]: mismatched types\n  --> /checkout/src/test/ui/associated-types/associated-types-path-2.rs:29:14\n   |\nLL |     f1(2i32, 4i32);\n   |              ^^^^ expected u32, found i32\n\n"}
[00:49:31] {"message":"aborting due to 6 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 6 previous errors\n\n"}
[00:49:31] {"message":"Some errors occurred: E0277, E0308.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0277, E0308.\n"}
[00:49:31] {"message":"For more information about an error, try `rustc --explain E0277`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0277`.\n"}
[00:49:31] ------------------------------------------
[00:49:31] 
[00:49:31] thread '[ui] ui/associated-types/associated-types-path-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:49:31] 
[00:49:31] 
[00:49:31] ---- [ui] ui/associated-types/associated-types-subtyping-1.rs stdout ----
[00:49:31] diff of stderr:
[00:49:31] 
[00:49:31] 1 error[E0623]: lifetime mismatch
[00:49:31] -   --> $DIR/associated-types-subtyping-1.rs:36:38
[00:49:31] -    |
[00:49:31] - LL | fn method2<'a,'b,T>(x: &'a T, y: &'b T)
[00:49:31] -    |                        -----     ----- these two types are declared with different lifetimes...
[00:49:31] - ...
[00:49:31] - LL |     let _c: <T as Trait<'b>>::Type = a; //~ ERROR E0623
[00:49:31] -    |                                      ^ ...but data from `y` flows into `x` here
[00:49:31] - error[E0623]: lifetime mismatch
[00:49:31] 11   --> $DIR/associated-types-subtyping-1.rs:45:38
[00:49:31] 12    |
[00:49:31] 12    |
[00:49:31] 13 LL | fn method3<'a,'b,T>(x: &'a T, y: &'b T)
[00:49:31] 
[00:49:31] 16 LL |     let _c: <T as Trait<'a>>::Type = b; //~ ERROR E0623
[00:49:31] 17    |                                      ^ ...but data from `y` flows into `x` here
[00:49:31] - error: aborting due to 2 previous errors
[00:49:31] + error: aborting due to previous error
[00:49:31] 20 
[00:49:31] 21 For more information about this error, try `rustc --explain E0623`.
[00:49:31] 21 For more information about this error, try `rustc --explain E0623`.
[00:49:31] 22 
[00:49:31] 
[00:49:31] 
[00:49:31] The actual stderr differed from the expected stderr.
[00:49:31] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-subtyping-1/associated-types-subtyping-1.stderr
[00:49:31] To update references, rerun the tests and pass the `--bless` flag
[00:49:31] To only update this specific test, also pass `--test-args associated-types/associated-types-subtyping-1.rs`
[00:49:31] error: 1 errors occurred comparing output.
[00:49:31] status: exit code: 1
[00:49:31] status: exit code: 1
[00:49:31] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-subtyping-1.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-subtyping-1/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-subtyping-1/auxiliary" "-A" "unused"
[00:49:31] ------------------------------------------
[00:49:31] 
[00:49:31] ------------------------------------------
[00:49:31] stderr:
[00:49:31] stderr:
[00:49:31] ------------------------------------------
[00:49:31] {"message":"lifetime mismatch","code":{"code":"E0623","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/associated-types/associated-types-subtyping-1.rs","byte_start":1166,"byte_end":1171,"line_start":39,"line_end":39,"column_start":34,"column_end":39,"is_primary":false,"text":[{"text":"fn method3<'a,'b,T>(x: &'a T, y: &'b T)","highlight_start":34,"highlight_end":39}],"label":"these two types are declared with different lifetimes...","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/associated-types/associated-types-subtyping-1.rs","byte_start":1156,"byte_end":1161,"line_start":39,"line_end":39,"column_start":24,"column_end":29,"is_primary":false,"text":[{"text":"fn method3<'a,'b,T>(x: &'a T, y: &'b T)","highlight_start":24,"highlight_end":29}],"label":"","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/associated-types/associated-types-subtyping-1.rs","byte_start":1387,"byte_end":1388,"line_start":45,"line_end":45,"column_start":38,"column_end":39,"is_primary":true,"text":[{"text":"    let _c: <T as Trait<'a>>::Type = b; //~ ERROR E0623","highlight_start":38,"highlight_end":39}],"label":"...but data from `y` flows into `x` here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0623]: lifetime mismatch\n  --> /checkout/src/test/ui/associated-types/associated-types-subtyping-1.rs:45:38\n   |\nLL | fn method3<'a,'b,T>(x: &'a T, y: &'b T)\n   |                        -----     ----- these two types are declared with different lifetimes...\n...\nLL |     let _c: <T as Trait<'a>>::Type = b; //~ ERROR E0623\n   |                                      ^ ...but data from `y` flows into `x` here\n\n"}
[00:49:31] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:49:31] {"message":"For more information about this error, try `rustc --explain E0623`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0623`.\n"}
[00:49:31] ------------------------------------------
[00:49:31] 
[00:49:31] thread '[ui] ui/associated-types/associated-types-subtyping-1.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:49:31] 
[00:49:31] 
[00:49:31] ---- [ui] ui/binop/binop-consume-args.rs stdout ----
[00:49:31] diff of stderr:
[00:49:31] 
[00:49:31] 1 error[E0382]: use of moved value: `lhs`
[00:49:31] +   --> $DIR/binop-consume-args.rs:47:10
[00:49:31] 3    |
[00:49:31] 3    |
[00:49:31] - LL |     lhs + rhs;
[00:49:31] + LL |     lhs & rhs;
[00:49:31] 5    |     --- value moved here
[00:49:31] 6 LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
[00:49:31] 7    |          ^^^ value used here after move
[00:49:31] 
[00:49:31] 9    = note: move occurs because `lhs` has type `A`, which does not implement the `Copy` trait
[00:49:31] 10 
[00:49:31] 11 error[E0382]: use of moved value: `rhs`
[00:49:31] +   --> $DIR/binop-consume-args.rs:48:10
[00:49:31] 13    |
[00:49:31] 13    |
[00:49:31] - LL |     lhs + rhs;
[00:49:31] + LL |     lhs & rhs;
[00:49:31] 15    |           --- value moved here
[00:49:31] 16 LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
[00:49:31] 17 LL |     drop(rhs);  //~ ERROR use of moved value: `rhs`
[00:49:31] 
[00:49:31] 20    = note: move occurs because `rhs` has type `B`, which does not implement the `Copy` trait
[00:49:31] 21 
[00:49:31] 22 error[E0382]: use of moved value: `lhs`
[00:49:31] +   --> $DIR/binop-consume-args.rs:53:10
[00:49:31] 24    |
[00:49:31] 24    |
[00:49:31] - LL |     lhs - rhs;
[00:49:31] + LL |     lhs | rhs;
[00:49:31] 26    |     --- value moved here
[00:49:31] 27 LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
[00:49:31] 28    |          ^^^ value used here after move
[00:49:31] 
[00:49:31] 30    = note: move occurs because `lhs` has type `A`, which does not implement the `Copy` trait
[00:49:31] 31 
[00:49:31] 32 error[E0382]: use of moved value: `rhs`
[00:49:31] +   --> $DIR/binop-consume-args.rs:54:10
[00:49:31] 34    |
[00:49:31] 34    |
[00:49:31] - LL |     lhs - rhs;
[00:49:31] + LL |     lhs | rhs;
[00:49:31] 36    |           --- value moved here
[00:49:31] 37 LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
[00:49:31] 38 LL |     drop(rhs);  //~ ERROR use of moved value: `rhs`
[00:49:31] 
[00:49:31] 41    = note: move occurs because `rhs` has type `B`, which does not implement the `Copy` trait
[00:49:31] 42 
[00:49:31] 43 error[E0382]: use of moved value: `lhs`
[00:49:31] +   --> $DIR/binop-consume-args.rs:59:10
[00:49:31] 45    |
[00:49:31] 45    |
[00:49:31] - LL |     lhs * rhs;
[00:49:31] + LL |     lhs ^ rhs;
[00:49:31] 47    |     --- value moved here
[00:49:31] 48 LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
[00:49:31] 49    |          ^^^ value used here after move
[00:49:31] 
[00:49:31] 51    = note: move occurs because `lhs` has type `A`, which does not implement the `Copy` trait
[00:49:31] 52 
[00:49:31] 53 error[E0382]: use of moved value: `rhs`
[00:49:31] +   --> $DIR/binop-consume-args.rs:60:10
[00:49:31] 55    |
[00:49:31] 55    |
[00:49:31] - LL |     lhs * rhs;
[00:49:31] + LL |     lhs ^ rhs;
[00:49:31] 57    |           --- value moved here
[00:49:31] 58 LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
[00:49:31] 59 LL |     drop(rhs);  //~ ERROR use of moved value: `rhs`
[00:49:31] 
[00:49:31] 62    = note: move occurs because `rhs` has type `B`, which does not implement the `Copy` trait
[00:49:31] 63 
[00:49:31] 64 error[E0382]: use of moved value: `lhs`
[00:49:31] +   --> $DIR/binop-consume-args.rs:65:10
[00:49:31] 66    |
[00:49:31] 66    |
[00:49:31] - LL |     lhs / rhs;
[00:49:31] + LL |     lhs << rhs;
[00:49:31] 68    |     --- value moved here
[00:49:31] 69 LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
[00:49:31] 70    |          ^^^ value used here after move
[00:49:31] 
[00:49:31] 72    = note: move occurs because `lhs` has type `A`, which does not implement the `Copy` trait
[00:49:31] 73 
[00:49:31] 74 error[E0382]: use of moved value: `rhs`
[00:49:31] +   --> $DIR/binop-consume-args.rs:66:10
[00:49:31] 76    |
[00:49:31] 76    |
[00:49:31] - LL |     lhs / rhs;
[00:49:31] -    |           --- value moved here
[00:49:31] + LL |     lhs << rhs;
[00:49:31] +    |            --- value moved here
[00:49:31] 79 LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
[00:49:31] 80 LL |     drop(rhs);  //~ ERROR use of moved value: `rhs`
[00:49:31] 81    |          ^^^ value used here after move
[00:49:31] 
[00:49:31] 83    = note: move occurs because `rhs` has type `B`, which does not implement the `Copy` trait
[00:49:31] 84 
[00:49:31] 85 error[E0382]: use of moved value: `lhs`
[00:49:31] +   --> $DIR/binop-consume-args.rs:71:10
[00:49:31] 87    |
[00:49:31] 87    |
[00:49:31] - LL |     lhs % rhs;
[00:49:31] + LL |     lhs >> rhs;
[00:49:31] 89    |     --- value moved here
[00:49:31] 90 LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
[00:49:31] 91    |          ^^^ value used here after move
[00:49:31] 
[00:49:31] 93    = note: move occurs because `lhs` has type `A`, which does not implement the `Copy` trait
[00:49:31] 94 
[00:49:31] 95 error[E0382]: use of moved value: `rhs`
[00:49:31] +   --> $DIR/binop-consume-args.rs:72:10
[00:49:31] 97    |
[00:49:31] 97    |
[00:49:31] - LL |     lhs % rhs;
[00:49:31] -    |           --- value moved here
[00:49:31] + LL |     lhs >> rhs;
[00:49:31] +    |            --- value moved here
[00:49:31] 100 LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
[00:49:31] 101 LL |     drop(rhs);  //~ ERROR use of moved value: `rhs`
[00:49:31] 102    |          ^^^ value used here after move
[00:49:31] 
[00:49:31] 104    = note: move occurs because `rhs` has type `B`, which does not implement the `Copy` trait
[00:49:31] 105 
[00:49:31] 106 error[E0382]: use of moved value: `lhs`
[00:49:31] +   --> $DIR/binop-consume-args.rs:29:10
[00:49:31] 108    |
[00:49:31] 108    |
[00:49:31] - LL |     lhs & rhs;
[00:49:31] + LL |     lhs * rhs;
[00:49:31] 110    |     --- value moved here
[00:49:31] 111 LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
[00:49:31] 112    |          ^^^ value used here after move
[00:49:31] 
[00:49:31] 114    = note: move occurs because `lhs` has type `A`, which does not implement the `Copy` trait
[00:49:31] 115 
[00:49:31] 116 error[E0382]: use of moved value: `rhs`
[00:49:31] +   --> $DIR/binop-consume-args.rs:30:10
[00:49:31] 118    |
[00:49:31] 118    |
[00:49:31] - LL |     lhs & rhs;
[00:49:31] + LL |     lhs * rhs;
[00:49:31] 120    |           --- value moved here
[00:49:31] 121 LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
[00:49:31] 122 LL |     drop(rhs);  //~ ERROR use of moved value: `rhs`
[00:49:31] 
[00:49:31] 125    = note: move occurs because `rhs` has type `B`, which does not implement the `Copy` trait
[00:49:31] 126 
[00:49:31] 127 error[E0382]: use of moved value: `lhs`
[00:49:31] +   --> $DIR/binop-consume-args.rs:35:10
[00:49:31] 129    |
[00:49:31] 129    |
[00:49:31] - LL |     lhs | rhs;
[00:49:31] + LL |     lhs / rhs;
[00:49:31] 131    |     --- value moved here
[00:49:31] 132 LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
[00:49:31] 133    |          ^^^ value used here after move
[00:49:31] 
[00:49:31] 135    = note: move occurs because `lhs` has type `A`, which does not implement the `Copy` trait
[00:49:31] 136 
[00:49:31] 137 error[E0382]: use of moved value: `rhs`
[00:49:31] +   --> $DIR/binop-consume-args.rs:36:10
[00:49:31] 139    |
[00:49:31] 139    |
[00:49:31] - LL |     lhs | rhs;
[00:49:31] + LL |     lhs / rhs;
[00:49:31] 141    |           --- value moved here
[00:49:31] 142 LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
[00:49:31] 143 LL |     drop(rhs);  //~ ERROR use of moved value: `rhs`
[00:49:31] 
[00:49:31] 146    = note: move occurs because `rhs` has type `B`, which does not implement the `Copy` trait
[00:49:31] 147 
[00:49:31] 148 error[E0382]: use of moved value: `lhs`
[00:49:31] +   --> $DIR/binop-consume-args.rs:41:10
[00:49:31] 150    |
[00:49:31] 150    |
[00:49:31] - LL |     lhs ^ rhs;
[00:49:31] + LL |     lhs % rhs;
[00:49:31] 152    |     --- value moved here
[00:49:31] 153 LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
[00:49:31] 154    |          ^^^ value used here after move
[00:49:31] 
[00:49:31] 156    = note: move occurs because `lhs` has type `A`, which does not implement the `Copy` trait
[00:49:31] 157 
[00:49:31] 158 error[E0382]: use of moved value: `rhs`
[00:49:31] +   --> $DIR/binop-consume-args.rs:42:10
[00:49:31] 160    |
[00:49:31] 160    |
[00:49:31] - LL |     lhs ^ rhs;
[00:49:31] + LL |     lhs % rhs;
[00:49:31] 162    |           --- value moved here
[00:49:31] 163 LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
[00:49:31] 164 LL |     drop(rhs);  //~ ERROR use of moved value: `rhs`
[00:49:31] 
[00:49:31] 167    = note: move occurs because `rhs` has type `B`, which does not implement the `Copy` trait
[00:49:31] 168 
[00:49:31] 169 error[E0382]: use of moved value: `lhs`
[00:49:31] +   --> $DIR/binop-consume-args.rs:23:10
[00:49:31] 171    |
[00:49:31] 171    |
[00:49:31] - LL |     lhs << rhs;
[00:49:31] + LL |     lhs - rhs;
[00:49:31] 173    |     --- value moved here
[00:49:31] 174 LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
[00:49:31] 175    |          ^^^ value used here after move
[00:49:31] 
[00:49:31] 177    = note: move occurs because `lhs` has type `A`, which does not implement the `Copy` trait
[00:49:31] 178 
[00:49:31] 179 error[E0382]: use of moved value: `rhs`
[00:49:31] +   --> $DIR/binop-consume-args.rs:24:10
[00:49:31] 181    |
[00:49:31] 181    |
[00:49:31] - LL |     lhs << rhs;
[00:49:31] -    |            --- value moved here
[00:49:31] + LL |     lhs - rhs;
[00:49:31] +    |           --- value moved here
[00:49:31] 184 LL |     drop(lhs);  //~ ERROR use of moved value: `lhs`
[00:49:31] 185 LL |     drop(rhs);  //~ ERROR use of moved value: `rhs`
[00:49:31] 186    |          ^^^ value used here after move
[00:49:31] 
[00:49:31] 188    = note: move occurs because `rhs` has type `B`, which does not implement the `Copy` trait
[00:49:31] 189 
[00:49:31] 190 error[E0382]: use of moved value: `lhs`
[00:49:31] +   --> $DIR/binop-consume-args.rs:17:10
[00:49:31] 192    |
[00:49:31] 192    |
---
[00:49:31] -   --> $DIR/issue-32829-2.rs:17:9
[00:49:31] + error[E0658]: statements in statics are unstable (see issue #48821)
[00:49:31] +   --> $DIR/issue-32829-2.rs:67:9
[00:49:31] 3    |
[00:49:31] 4 LL |         5;
[00:49:31] 
[00:49:31] 6    |
[00:49:31] 7    = help: add #![feature(const_let)] to the crate attributes to enable
[00:49:31] 8 
[00:49:31] 8 
[00:49:31] - error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
[00:49:31] -   --> $DIR/issue-32829-2.rs:25:9
[00:49:31] + error[E0015]: calls in statics are limited to constant functions, tuple structs and tuple variants
[00:49:31] +   --> $DIR/issue-32829-2.rs:75:9
[00:49:31] 11    |
[00:49:31] 12 LL |         invalid();
[00:49:31] 
[00:49:31] 14 
[00:49:31] - error[E0658]: statements in constants are unstable (see issue #48821)
[00:49:31] -   --> $DIR/issue-32829-2.rs:25:9
[00:49:31] -   --> $DIR/issue-32829-2.rs:25:9
[00:49:31] + error[E0658]: statements in statics are unstable (see issue #48821)
[00:49:31] +   --> $DIR/issue-32829-2.rs:75:9
[00:49:31] 17    |
[00:49:31] 18 LL |         invalid();
[00:49:31] 
[00:49:31] 20    |
[00:49:31] 21    = help: add #![feature(const_let)] to the crate attributes to enable
[00:49:31] 22 
[00:49:31] 22 
[00:49:31] - error[E0658]: statements in constants are unstable (see issue #48821)
[00:49:31] -   --> $DIR/issue-32829-2.rs:34:9
[00:49:31] + error[E0658]: statements in statics are unstable (see issue #48821)
[00:49:31] +   --> $DIR/issue-32829-2.rs:84:9
[00:49:31] 25    |
[00:49:31] 26 LL |         valid();
[00:49:31] 
[00:49:31] 58    |
[00:49:31] 59    = help: add #![feature(const_let)] to the crate attributes to enable
[00:49:31] 60 
[00:49:31] 60 
[00:49:31] - error[E0658]: statements in statics are unstable (see issue #48821)
[00:49:31] -   --> $DIR/issue-32829-2.rs:67:9
[00:49:31] + error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
[00:49:31] +   --> $DIR/issue-32829-2.rs:25:9
[00:49:31] 63    |
[00:49:31] - LL |         5;
[00:49:31] -    |
[00:49:31] -    = help: add #![feature(const_let)] to the crate attributes to enable
[00:49:31] - 
[00:49:31] - 
[00:49:31] - error[E0015]: calls in statics are limited to constant functions, tuple structs and tuple variants
[00:49:31] -    |
[00:49:31] -    |
[00:49:31] 72 LL |         invalid();
[00:49:31] 74 
[00:49:31] 
[00:49:31] - error[E0658]: statements in statics are unstable (see issue #48821)
[00:49:31] -   --> $DIR/issue-32829-2.rs:75:9
[00:49:31] -   --> $DIR/issue-32829-2.rs:75:9
[00:49:31] + error[E0658]: statements in constants are unstable (see issue #48821)
[00:49:31] +   --> $DIR/issue-32829-2.rs:25:9
[00:49:31] 77    |
[00:49:31] 78 LL |         invalid();
[00:49:31] 
[00:49:31] 80    |
[00:49:31] 81    = help: add #![feature(const_let)] to the crate attributes to enable
[00:49:31] 82 
[00:49:31] 82 
[00:49:31] - error[E0658]: statements in statics are unstable (see issue #48821)
[00:49:31] -   --> $DIR/issue-32829-2.rs:84:9
[00:49:31] + error[E0658]: statements in constants are unstable (see issue #48821)
[00:49:31] +   --> $DIR/issue-32829-2.rs:34:9
[00:49:31] 85    |
[00:49:31] 86 LL |         valid();
[00:49:31] 
[00:49:31] +    |
[00:49:31] +    = help: add #![feature(const_let)] to the crate attributes to enable
[00:49:31] + 
[00:49:31] + 
[00:49:31] + error[E0658]: statements in constants are unstable (see issue #48821)
[00:49:31] +   --> $DIR/issue-32829-2.rs:17:9
[00:49:31] +    |
[00:49:31] + LL |         5;
[00:49:31] 88    |
[00:49:31] 89    = help: add #![feature(const_let)] to the crate attributes to enable
[00:49:31] 90 
[00:49:31] 
[00:49:31] 
[00:49:31] 
[00:49:31] The actual stderr differed from the expected stderr.
[00:49:31] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32829-2/issue-32829-2.stderr
[00:49:31] To update references, rerun the tests and pass the `--bless` flag
[00:49:31] To only update this specific test, also pass `--test-args issues/issue-32829-2.rs`
[00:49:31] error: 1 errors occurred comparing output.
[00:49:31] status: exit code: 1
[00:49:31] status: exit code: 1
[00:49:31] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-32829-2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32829-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32829-2/auxiliary" "-A" "unused"
[00:49:31] ------------------------------------------
[00:49:31] 
[00:49:31] ------------------------------------------
[00:49:31] stderr:
[00:49:31] stderr:
[00:49:31] ------------------------------------------
[00:49:31] {"message":"statements in statics are unstable (see issue #48821)","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n
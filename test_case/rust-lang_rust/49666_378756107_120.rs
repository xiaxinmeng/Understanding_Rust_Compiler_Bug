\n\nWith this approach, x and y share ownership of the data via the `Rc` (reference\ncount type). `RefCell` essentially performs runtime borrow checking: ensuring\nthat at most one writer or multiple readers can access the data at any one time.\n\nIf you wish to learn more about ownership in Rust, start with the chapter in the\nBook:\n\nhttps://doc.rust-lang.org/book/first-edition/ownership.html\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/moves-based-on-type-tuple.rs","byte_start":619,"byte_end":620,"line_start":16,"line_end":16,"column_start":13,"column_end":14,"is_primary":true,"text":[{"text":"    box (x, x)","highlight_start":13,"highlight_end":14}],"label":"value used here after move","suggested_replacement":null,"expansion":null},{"file_name":"/checkout/src/test/ui/moves-based-on-type-tuple.rs","byte_start":616,"byte_end":617,"line_start":16,"line_end":16,"column_start":10,"column_end":11,"is_primary":false,"text":[{"text":"    box (x, x)","highlight_start":10,"highlight_end":11}],"label":"value moved here","suggested_replacement":null,"expansion":null}],"children":[{"message":"move occurs because `x` has type `std::boxed::Box<isize>`, which does not implement the `Copy` trait","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0382]: use of moved value: `x` (Ast)\n  --> /checkout/src/test/ui/moves-based-on-type-tuple.rs:16:13\n   |\nLL |     box (x, x)\n   |          -  ^ value used here after move\n   |          |\n   |          value moved here\n   |\n   = note: move occurs because `x` has type `std::boxed::Box<isize>`, which does not implement the `Copy` trait\n\n"}
[00:42:43] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous err   --- immutable borrow occurs here
[00:42:43]
[00:42:43] - ...
[00:42:43] - LL |                 map.set(String::new()); // Just AST errors here
[00:42:43] + LL |             Some(v) => {
[00:42:43] + LL |                 map.set(String::new()); // Both AST and MIR error here
[00:42:43] 32    |                 ^^^ mutable borrow occurs here
[00:42:43] 33 ...
[00:42:43] 34 LL | }
[00:42:43]
[00:42:43] 35    | - immutable borrow ends here
[00:42:43] 36
[00:42:43] - error[E0502]: cannot borrow `*map` as mutable because it is also borrowed as immutable (Mir)
[00:42:43] -   --> $DIR/get_default.rs:44:17
[00:42:43] + error[E0502]: cannot borrow `*map` as mutable because it is also borrowed as immutable (Ast)
[00:42:43] +   --> $DIR/get_default.rs:50:17
[00:42:43] 39    |
[00:42:43] 40 LL |         match map.get() {
[00:42:43] 41    |               --- immutable borrow occurs here
[00:42:43]
[00:42:43] - LL |             Some(v) => {
[00:42:43] - LL |                 map.set(String::new()); // Both AST and MIR error here
[00:42:43] + ...
[00:42:43] + LL |                 map.set(String::new()); // Just AST errors here
[00:42:43] 44    |                 ^^^ mutable borrow occurs here
[00:42:43] 45 ...
[00:42:43] - LL |                 return v;
[00:42:43] -    |                        - borrow later used here
[00:42:43] + LL | }
[00:42:43] +    | - immutable borrow ends here
---
[00:42:43] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'nll/get_default.rs'
[00:42:43]
[00:42:43] error: 1 errors occurred comparing output.
[00:42:43] status: exit code: 101
[00:42:43] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/get_default.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/get_default.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Znll" "-Zborrowck=compare" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/get_default.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:42:43] {"message":"cannot borrow `*map` as mutable because it is also borrowed as immutable (Ast)","code":{"code":"E0502","explanation":"\nThis error indicates that you are trying to borrow a variable as mutable when it\nhas already been borrowed as immutable.\n\nExample of erroneous code:\n\n
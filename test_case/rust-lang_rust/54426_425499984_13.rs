\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/borrowck/borrowck-closures-mut-and-imm.rs","byte_start":1690,"byte_end":1692,"line_start":62,"line_end":62,"column_start":14,"column_end":16,"is_primary":false,"text":[{"text":"    let c1 = || get(&x);","highlight_start":14,"highlight_end":16}],"label":"borrow of `x` occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion"describe-lvalue.mir/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-describe-lvalue.mir/auxiliary" "-A" "unused"
[00:48:10] ------------------------------------------
[00:48:10] 
[00:48:10] ------------------------------------------
[00:48:10] stderr:
[00:48:10] stderr:
[00:48:10] ------------------------------------------
[00:48:10] {"message":"cannot borrow `_` as mutable more than once at a time","code":{"code":"E0499","explanation":"\nA variable was borrowed as mutable more than once. Erroneous code example:\n\n
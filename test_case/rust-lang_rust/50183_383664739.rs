plain
[00:58:25] running 1367 tests
[00:58:31] .................................................................................i..................
[00:58:39] .........................i..........................................................................
[00:58:44] ....................................................................................................
[00:58:48] .......F............................................................................................
[00:58:58] ....................................................................................................
[00:59:05] ....................................................................................................
[00:59:05] ....................................................................................................
[00:59:12] ..........F.........................................................................................
[00:59:28] ...........................i........................................................................
[00:59:35] ...i................................................................................................
[00:59:43] .................ii.................................................................................
[00:59:52] ..................................................................................................i.
[00:59:52] ..................................................................................................i.
[00:59:58] .......................F...F.......................................
[00:59:58] 
[00:59:58] ---- [ui] ui/error-codes/E0275.rs stdout ----
[00:59:58]  
[00:59:58]  
[00:59:58] error: ui test compiled successfully!
[00:59:58] status: exit code: 0
[00:59:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0275.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0275.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0275.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:59:58] ------------------------------------------
[00:59:58] 
[00:59:58] ------------------------------------------
[00:59:58] stderr:
---
[00:59:58] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:59:58] 
[00:59:58] ---- [ui] ui/issue-24424.rs stdout ----
[00:59:58]  
[00:59:58] error: ui test compiled successfully!
[00:59:58] status: exit code: 0
[00:59:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-24424.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-24424.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-24424.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:59:58] ------------------------------------------
[00:59:58] 
[00:59:58] ------------------------------------------
[00:59:58] stderr:
---
[00:59:58] 
[00:59:58] ---- [ui] ui/type-check-defaults.rs stdout ----
[00:59:58]  diff of stderr:
[00:59:58] 
[00:59:58] 24 LL | struct Foo<T, U: FromIterator<T>>(T, U);
[00:59:58] 26 
[00:59:58] 26 
[00:59:58] - error[E0277]: the trait bound `std::string::String: std::marker::Copy` is not satisfied
[00:59:58] -    |
[00:59:58] -    |
[00:59:58] - LL | struct Bounds<T:Copy=String>(T);
[00:59:58] -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::marker::Copy` is not implemented for `std::string::String`
[00:59:58] -    = note: required by `std::marker::Copy`
[00:59:58] - 
[00:59:58] - 
[00:59:58] - error[E0277]: the trait bound `std::string::String: std::marker::Copy` is not satisfied
[00:59:58] -    |
[00:59:58] -    |
[00:59:58] - LL | struct WhereClause<T=String>(T) where T: Copy;
[00:59:58] -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::marker::Copy` is not implemented for `std::string::String`
[00:59:58] -    = note: required by `std::marker::Copy`
[00:59:58] - 
[00:59:58] - 
[00:59:58] - error[E0277]: the trait bound `std::string::String: std::marker::Copy` is not satisfied
[00:59:58] -    |
[00:59:58] -    |
[00:59:58] - LL | trait TraitBound<T:Copy=String> {}
[00:59:58] -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::marker::Copy` is not implemented for `std::string::String`
[00:59:58] -    = note: required by `std::marker::Copy`
[00:59:58] - 
[00:59:58] - 
[00:59:58] 51 error[E0277]: the trait bound `T: std::marker::Copy` is not satisfied
[00:59:58] 53    |
[00:59:58] 
[00:59:58] 
[00:59:58] 68    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `i32 + u8`
[00:59:58] 69    |
[00:59:58] 70    = help: the trait `std::ops::Add<u8>` is not implemented for `i32`
[00:59:58] -    = note: required by `std::ops::Add`
[00:59:58] - error: aborting due to 7 previous errors
[00:59:58] + error: aborting due to 4 previous errors
[00:59:58] 74 
[00:59:58] 75 For more information about this error, try `rustc --explain E0277`.
[00:59:58] 75 For more information about this error, try `rustc --explain E0277`.
[00:59:58] 76 
[00:59:58] 
[00:59:58] 
[00:59:58] The actual stderr differed from the expected stderr.
[00:59:58] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-check-defaults.stderr
[00:59:58] To update references, run this command from build directory:
[00:59:58] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'type-check-defaults.rs'
[00:59:58] error: 1 errors occurred comparing output.
[00:59:58] status: exit code: 101
[00:59:58] status: exit code: 101
[00:59:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-check-defaults.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-check-defaults.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-check-defaults.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:59:58] ------------------------------------------
[00:59:58] 
[00:59:58] ------------------------------------------
[00:59:58] stderr:
[00:59:58] stderr:
[00:59:58] ------------------------------------------
[00:59:58] {"message":"the trait bound `i32: std::iter::FromIterator<i32>` is not satisfied","code":{"code":"E0277","explanation":"\nYou tried to use a type which doesn't implement some trait in a place which\nexpected that trait. Erroneous code example:\n\n
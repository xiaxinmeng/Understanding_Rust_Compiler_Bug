plain
[00:50:24] ....................................................................................................
[00:50:27] ....................................................................................................
[00:50:29] ....................................................................................................
[00:50:31] ....................................................................................................
[00:50:35] ....................................................................................F...............
[00:50:38] ....................................................................................................
[00:50:40] ...........................i......................F.................................................
[00:50:45] ....................................................................................................
[00:50:45] ....................................................................................................
[00:50:48] ..................................................................................F.................
[00:50:51] ..........F............F............................................................................
[00:50:54] .............................................................................................F......
[00:50:57] ....................................................................................................
[00:51:00] ...........................................F........................................................
[00:51:07] ................................i...................................................................
[00:51:07] ................................i...................................................................
[00:51:10] ..................F......F...................F.......F..............................................
[00:51:14] ...........F..................................................................F.....................
[00:51:20] ..............................................i....
[00:51:20] failures:
[00:51:20] 
[00:51:20] ---- [ui] ui/impl-trait/universal_wrong_bounds.rs stdout ----
[00:51:20] ---- [ui] ui/impl-trait/universal_wrong_bounds.rs stdout ----
[00:51:20] diff of stderr:
[00:51:20] 
[00:51:20] 3    |
[00:51:20] 4 LL | fn wants_debug(g: impl Debug) { } //~ ERROR cannot find
[00:51:20] - help: possible candidate is found in another module, you can import it into scope
[00:51:20] -    |
[00:51:20] -    |
[00:51:20] - LL | use std::fmt::Debug;
[00:51:20] 10 
[00:51:20] 11 error[E0405]: cannot find trait `Debug` in this scope
[00:51:20] 12   --> $DIR/universal_wrong_bounds.rs:20:26
[00:51:20] 
[00:51:20] 
[00:51:20] 13    |
[00:51:20] 14 LL | fn wants_display(g: impl Debug) { } //~ ERROR cannot find
[00:51:20] - help: possible candidate is found in another module, you can import it into scope
[00:51:20] -    |
[00:51:20] -    |
[00:51:20] - LL | use std::fmt::Debug;
[00:51:20] 20 
[00:51:20] 21 error: aborting due to 2 previous errors
[00:51:20] 22 
[00:51:20] 
[00:51:20] 
[00:51:20] 
[00:51:20] The actual stderr differed from the expected stderr.
[00:51:20] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/universal_wrong_bounds/universal_wrong_bounds.stderr
[00:51:20] To update references, rerun the tests and pass the `--bless` flag
[00:51:20] To only update this specific test, also pass `--test-args impl-trait/universal_wrong_bounds.rs`
[00:51:20] error: 1 errors occurred comparing output.
[00:51:20] status: exit code: 1
[00:51:20] status: exit code: 1
[00:51:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/universal_wrong_bounds.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/universal_wrong_bounds/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/universal_wrong_bounds/auxiliary" "-A" "unused"
[00:51:20] ------------------------------------------
[00:51:20] 
[00:51:20] ------------------------------------------
[00:51:20] stderr:
[00:51:20] stderr:
[00:51:20] ------------------------------------------
[00:51:20] {"message":"cannot find trait `Debug` in this scope","code":{"code":"E0405","explanation":"\nThe code refers to a trait that is not in scope.\n\nErroneous code example:\n\n
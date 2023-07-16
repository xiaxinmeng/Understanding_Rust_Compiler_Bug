plain

---- [ui] ui/lto/lto-duplicate-symbols.rs stdout ----
diff of stderr:

1 warning: Linking globals named 'foo': symbol multiply defined!
2 
- error: failed to load bc of "lto-duplicate-symbols2.lto_duplicate_symbols2.HASH-cgu.0.rcgu.o": 
+ error: failed to load bitcode of module "lto-duplicate-symbols2.lto_duplicate_symbols2.HASH-cgu.0.rcgu.o": 
5 error: aborting due to previous error; 1 warning emitted
6 


---
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lto/lto-duplicate-symbols.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto/lto-duplicate-symbols" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "lto" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto/lto-duplicate-symbols/auxiliary"
stdout: none
--- stderr -------------------------------
warning: Linking globals named 'foo': symbol multiply defined!

error: failed to load bitcode of module "lto-duplicate-symbols2.lto_duplicate_symbols2.838f7f19-cgu.0.rcgu.o": 
error: aborting due to previous error; 1 warning emitted
------------------------------------------



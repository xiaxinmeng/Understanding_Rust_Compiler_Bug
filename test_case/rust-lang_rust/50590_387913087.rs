plain
[00:41:07] ....................................................................................................
[00:41:12] ....................................................................................................
[00:41:18] ..........................................i.........................................................
[00:41:23] ..................i.................................................................................
[00:41:28] .....................................ii.F...........................................................
[00:41:40] .................i....................................................................
[00:41:40] failures:
[00:41:40] 
[00:41:40] ---- [ui] ui/rfc-2093-infer-outlives/enum.rs stdout ----
[00:41:40] ---- [ui] ui/rfc-2093-infer-outlives/enum.rs stdout ----
[00:41:40]  diff of stderr:
[00:41:40] 
[00:41:40] 3    |
[00:41:40] 4 LL | struct Bar<'b, U> {
[00:41:40] 5    |                - help: consider adding an explicit lifetime bound `U: 'b`...
[00:41:40] - LL |     field2: &'b U //~ ERROR 23:5: 23:18: the parameter type `U` may not live long enough [E0309]
[00:41:40] + LL |     field2: &'b U //~ ERROR the parameter type `U` may not live long enough [E0309]
[00:41:40] 8    |
[00:41:40] 8    |
[00:41:40] 9 note: ...so that the reference type `&'b U` does not outlive the data it points at
[00:41:40] 10   --> $DIR/enum.rs:23:5
[00:41:40] 11    |
[00:41:40] 11    |
[00:41:40] - LL |     field2: &'b U //~ ERROR 23:5: 23:18: the parameter type `U` may not live long enough [E0309]
[00:41:40] + LL |     field2: &'b U //~ ERROR the parameter type `U` may not live long enough [E0309]
[00:41:40] 14 
[00:41:40] 14 
[00:41:40] 15 error[E0309]: the parameter type `K` may not live long enough
[00:41:40] 17    |
[00:41:40] 17    |
[00:41:40] 18 LL | enum Ying<'c, K> {
[00:41:40] 19    |               - help: consider adding an explicit lifetime bound `K: 'c`...
[00:41:40] - LL |     One(&'c Yang<K>) //~ ERROR 30:9: 30:21: the parameter type `K` may not live long enough [E0309]
[00:41:40] + LL |     One(&'c Yang<K>) //~ ERROR the parameter type `K` may not live long enough [E0309]
[00:41:40] 22    |
[00:41:40] 22    |
[00:41:40] 23 note: ...so that the reference type `&'c Yang<K>` does not outlive the data it points at
[00:41:40] 24   --> $DIR/enum.rs:30:9
[00:41:40] 25    |
[00:41:40] 25    |
[00:41:40] - LL |     One(&'c Yang<K>) //~ ERROR 30:9: 30:21: the parameter type `K` may not live long enough [E0309]
[00:41:40] + LL |     One(&'c Yang<K>) //~ ERROR the parameter type `K` may not live long enough [E0309]
[00:41:40] 28 
[00:41:40] 29 error: aborting due to 2 previous errors
[00:41:40] 
[00:41:40] 
[00:41:40] 
[00:41:40] The actual stderr differed from the expected stderr.
[00:41:40] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/enum.stderr
[00:41:40] To update references, run this command from build directory:
[00:41:40] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'rfc-2093-infer-outlives/enum.rs'
[00:41:40] 
[00:41:40] error: 1 errors occurred comparing output.
[00:41:40] status: exit code: 101
[00:41:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2093-infer-outlives/enum.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/enum.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/enum.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:41:40] ------------------------------------------
[00:41:40] 
[00:41:40] ------------------------------------------
[00:41:40] stderr:
[00:41:40] stderr:
[00:41:40] ------------------------------------------
[00:41:40] {"message":"the parameter type `U` may not live long enough","code":{"code":"E0309","explanation":"\nTypes in type definitions have lifetimes associated with them that represent\nhow long the data stored within them is guaranteed to be live. This lifetime\nmust be as long as the data needs to be alive, and missing the constraint that\ndenotes this will cause this error.\n\n
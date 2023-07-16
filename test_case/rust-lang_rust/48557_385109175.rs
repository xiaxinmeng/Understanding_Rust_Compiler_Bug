plain
[00:42:20] ..............................i.....................................................................
[00:42:24] ....................................................................................................
[00:42:28] ....................................................................................................
[00:42:31] ....................................................................................................
[00:42:35] ........................................................F...........................................
[00:42:46] ....................................................................................................
[00:42:46] ....................................................................................................
[00:42:52] ...F................................................................................................
[00:43:03] ............i.......................................................................................
[00:43:09] ............................ii......................................................................
[00:43:15] ....................................................................................................
[00:43:15] ....................................................................................................
t.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate-trivial_bounds-lint.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:43:22] ------------------------------------------
[00:43:22] 
[00:43:22] ------------------------------------------
[00:43:22] stderr:
---
[00:43:22] 
[00:43:22] ---- [ui] ui/issue-48728.rs stdout ----
[00:43:22]  diff of stderr:
[00:43:22] 
[00:43:22] 6 ...
[00:43:22] 7 LL | impl<T: Clone + ?Sized> Clone for Node<[T]> {
[00:43:22] 8    | ------------------------------------------- first implementation here
[00:43:22] +    |
[00:43:22] +    = note: upstream crates may add new impl of trait `std::clone::Clone` for type `[_]` in future versions
[00:43:22] 10 error: aborting due to previous error
[00:43:22] 11 
[00:43:22] 
[00:43:22] 
[00:43:22] 
[00:43:22] The actual stderr differed from the expected stderr.
[00:43:22] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-48728.stderr
[00:43:22] To update references, run this command from build directory:
[00:43:22] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'issue-48728.rs'
[00:43:22] 
[00:43:22] error: 1 errors occurred comparing output.
[00:43:22] status: exit code: 101
[00:43:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-48728.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-48728.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-48728.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:43:22] ------------------------------------------
[00:43:22] 
[00:43:22] ------------------------------------------
[00:43:22] stderr:
[00:43:22] stderr:
[00:43:22] ------------------------------------------
[00:43:22] {"message":"conflicting implementations of trait `std::clone::Clone` for type `Node<[_]>`:","code":{"code":"E0119","explanation":"\nThere are conflicting trait implementations for the same type.\nExample of erroneous code:\n\n
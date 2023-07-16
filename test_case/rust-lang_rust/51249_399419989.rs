plain
[00:42:45] ....................................................................................................
[00:42:49] ....................................................................................................
[00:42:55] ....................................................................................................
[00:43:00] ....................................................................................................
[00:43:06] ........i....................................................F...........................i..........
[00:43:16] ....................................................................................................
[00:43:22] ....................................................................................................
[00:43:28] ..........................i.........................................................................
[00:43:29] ...........
[00:43:29] ...........
[00:43:29] failures:
[00:43:29] 
[00:43:29] ---- [ui] ui/nll/issue-51244.rs stdout ----
[00:43:29] diff of stderr:
[00:43:29] 
[00:43:29] - error[E0594]: cannot assign to data in a `&` reference
[00:43:29] + error[E0594]: cannot assign to `*my_ref` which is behind a `&` reference
[00:43:29] 2   --> $DIR/issue-51244.rs:15:5
[00:43:29] 3    |
[00:43:29] - LL |     let ref my_ref @ _ = 0;
[00:43:29] -    |         -------------- help: consider changing this to be a mutable reference: `&mut ef my_ref @ _`
[00:43:29] 6 LL |     *my_ref = 0; //~ ERROR cannot assign to immutable borrowed content `*my_ref` [E0594]
[00:43:29] +    |     ^^^^^^^^^^^ cannot assign
[00:43:29] 8 
[00:43:29] 9 error: aborting due to previous error
[00:43:29] 10 
[00:43:29] 10 
[00:43:29] 
[00:43:29] 
[00:43:29] The actual stderr differed from the expected stderr.
[00:43:29] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-51244/issue-51244.stderr
[00:43:29] To update references, rerun the tests and pass the `--bless` flag
[00:43:29] To only update this specific test, also pass `--test-args nll/issue-51244.rs`
[00:43:29] error: 1 errors occurred comparing output.
[00:43:29] status: exit code: 101
[00:43:29] status: exit code: 101
[00:43:29] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-51244.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-51244/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-51244/auxiliary" "-A" "unused"
[00:43:29] ------------------------------------------
[00:43:29] 
[00:43:29] ------------------------------------------
[00:43:29] stderr:
[00:43:29] stderr:
[00:43:29] ------------------------------------------
[00:43:29] {"message":"cannot assign to `*my_ref` which is behind a `&` reference","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/issue-51244.rs","byte_start":529,"byte_end":540,"line_start":15,"line_end":15,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    *my_ref = 0; //~ ERROR cannot assign to immutable borrowed content `*my_ref` [E0594]","highlight_start":5,"highlight_end":16}],"label":"cannot assign","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0594]: cannot assign to `*my_ref` which is behind a `&` reference\n  --> /checkout/src/test/ui/nll/issue-51244.rs:15:5\n   |\nLL |     *my_ref = 0; //~ ERROR cannot assign to immutable borrowed content `*my_ref` [E0594]\n   |     ^^^^^^^^^^^ cannot assign\n\n"}
[00:43:29] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:43:29] {"message":"For more information about this error, try `rustc --explain E0594`.","code":null,"level":56 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu
56092 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/bin
51360 ./obj/build/x86_64-unknown-linux-gnu/stage0/bin
50796 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps
48708 ./src/test

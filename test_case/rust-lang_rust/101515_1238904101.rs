plain

---- [ui] src/test/ui/parser/issue-101477-let.rs stdout ----
diff of fixed:

4     let x = 2; //~ ERROR expect `=`, found `==`
5     println!("x: {}", x)
+ 


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-101477-let/issue-101477-let.fixed
To only update this specific test, also pass `--test-args parser/issue-101477-let.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-101477-let.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-101477-let" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-101477-let/auxiliary"
stdout: none
--- stderr -------------------------------
error: expect `=`, found `==`
   |
   |
LL |     let x == 2; //~ ERROR expect `=`, found `==`
   |           ^^ help: write `=` instead of `==`
error: aborting due to previous error
------------------------------------------



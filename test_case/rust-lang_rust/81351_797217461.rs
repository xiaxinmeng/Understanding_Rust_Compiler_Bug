
failures:

---- [ui] ui/const-generics/issues/issue-69654.rs stdout ----
diff of stderr:

4	LL | impl<T> Bar<T> for [u8; T] {}
5	   |                         ^ not a value
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
6	
-	error[E0599]: no function or associated item named `foo` found for struct `Foo<{_: usize}>` in the current scope
+	error[E0599]: the function or associated item `foo` exists for struct `Foo<{_: usize}>`, but its trait bounds were not satisfied
8	  --> $DIR/issue-69654.rs:17:10
9	   |
10	LL | struct Foo<const N: usize> {}

11	   | -------------------------- function or associated item `foo` not found for this
12	...
13	LL |     Foo::foo();
-	   |          ^^^ function or associated item not found in `Foo<{_: usize}>`
+	   |          ^^^ function or associated item cannot be called on `Foo<{_: usize}>` due to unsatisfied trait bounds
15	   |
-	   = note: the method `foo` exists but the following trait bounds were not satisfied:
+	   = note: the following trait bounds were not satisfied:
17	           `[u8; _]: Bar<[(); _]>`
18	
19	error: aborting due to 2 previous errors


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-69654/issue-69654.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/issues/issue-69654.rs`

error: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-69654.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-69654" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-69654/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error[E0423]: expected value, found type parameter `T`
  --> /checkout/src/test/ui/const-generics/issues/issue-69654.rs:5:25
   |
LL | impl<T> Bar<T> for [u8; T] {}
   |                         ^ not a value

error[E0599]: the function or associated item `foo` exists for struct `Foo<{_: usize}>`, but its trait bounds were not satisfied
  --> /checkout/src/test/ui/const-generics/issues/issue-69654.rs:17:10
   |
LL | struct Foo<const N: usize> {}
   | -------------------------- function or associated item `foo` not found for this
...
LL |     Foo::foo();
   |          ^^^ function or associated item cannot be called on `Foo<{_: usize}>` due to unsatisfied trait bounds
   |
   = note: the following trait bounds were not satisfied:
           `[u8; _]: Bar<[(); _]>`

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0423, E0599.
For more information about an error, try `rustc --explain E0423`.

------------------------------------------


---- [ui] ui/const-generics/issues/issue-69654-run-pass.rs stdout ----
diff of stderr:

-	error[E0599]: no function or associated item named `foo` found for struct `Foo<{_: usize}>` in the current scope
+	error[E0599]: the function or associated item `foo` exists for struct `Foo<{_: usize}>`, but its trait bounds were not satisfied
2	  --> $DIR/issue-69654-run-pass.rs:16:10
3	   |
4	LL | struct Foo<const N: usize> {}

5	   | -------------------------- function or associated item `foo` not found for this
6	...
7	LL |     Foo::foo();
-	   |          ^^^ function or associated item not found in `Foo<{_: usize}>`
+	   |          ^^^ function or associated item cannot be called on `Foo<{_: usize}>` due to unsatisfied trait bounds
9	   |
-	   = note: the method `foo` exists but the following trait bounds were not satisfied:
+	   = note: the following trait bounds were not satisfied:
11	           `[u8; _]: Bar<[(); _]>`
12	
13	error: aborting due to previous error


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-69654-run-pass/issue-69654-run-pass.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/issues/issue-69654-run-pass.rs`

error: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-69654-run-pass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-69654-run-pass" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-69654-run-pass/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error[E0599]: the function or associated item `foo` exists for struct `Foo<{_: usize}>`, but its trait bounds were not satisfied
  --> /checkout/src/test/ui/const-generics/issues/issue-69654-run-pass.rs:16:10
   |
LL | struct Foo<const N: usize> {}
   | -------------------------- function or associated item `foo` not found for this
...
LL |     Foo::foo();
   |          ^^^ function or associated item cannot be called on `Foo<{_: usize}>` due to unsatisfied trait bounds
   |
   = note: the following trait bounds were not satisfied:
           `[u8; _]: Bar<[(); _]>`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.

------------------------------------------



failures:
    [ui] ui/const-generics/issues/issue-69654-run-pass.rs
    [ui] ui/const-generics/issues/issue-69654.rs

test result: FAILED. 11239 passed; 2 failed; 88 ignored; 0 measured; 0 filtered out; finished in 115.27s

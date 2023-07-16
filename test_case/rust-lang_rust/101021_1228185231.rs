plain
.........................i.....................................................i........ 3080/13432
........................................................................................ 3168/13432
........................................................................................ 3256/13432
.......................................................iiiii............................ 3344/13432
.................................F..FF.................................................. 3432/13432
........................................................................................ 3608/13432
........................................................................................ 3696/13432
........................................................................................ 3784/13432
.........................................................................i..........i... 3872/13432
---

4 LL |     let ft =
5    |         ^^
6    |
-    = note: overflowed on FingerTree<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<i32>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
+    = note: overflowed on overflowed on FingerTree<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<i32>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
9 error: aborting due to previous error
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dropck/dropck_no_diverge_on_nonregular_1/dropck_no_diverge_on_nonregular_1.stderr
To only update this specific test, also pass `--test-args dropck/dropck_no_diverge_on_nonregular_1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dropck/dropck_no_diverge_on_nonregular_1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dropck/dropck_no_diverge_on_nonregular_1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dropck/dropck_no_diverge_on_nonregular_1/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0320]: overflow while adding drop-check rules for FingerTree<i32>
   |
   |
LL |     let ft = //~ ERROR overflow while adding drop-check rules for FingerTree
   |
   |
   = note: overflowed on overflowed on FingerTree<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<i32>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/dropck/dropck_no_diverge_on_nonregular_3.rs stdout ----
diff of stderr:

4 LL |     let w =
5    |         ^
6    |
-    = note: overflowed on FingerTree<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<u32>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
+    = note: overflowed on overflowed on FingerTree<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<u32>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
8 
9 error[E0320]: overflow while adding drop-check rules for Wrapper<u32>


12 LL |         Some(Wrapper::Simple::<u32>);
14    |
14    |
-    = note: overflowed on FingerTree<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<u32>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
+    = note: overflowed on overflowed on FingerTree<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<u32>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
17 error: aborting due to 2 previous errors
18 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dropck/dropck_no_diverge_on_nonregular_3/dropck_no_diverge_on_nonregular_3.stderr
To only update this specific test, also pass `--test-args dropck/dropck_no_diverge_on_nonregular_3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dropck/dropck_no_diverge_on_nonregular_3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dropck/dropck_no_diverge_on_nonregular_3" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dropck/dropck_no_diverge_on_nonregular_3/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0320]: overflow while adding drop-check rules for Option<Wrapper<u32>>
   |
   |
LL |     let w = //~ ERROR overflow while adding drop-check rules for Option
   |
   |
   = note: overflowed on overflowed on FingerTree<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<u32>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

error[E0320]: overflow while adding drop-check rules for Wrapper<u32>
   |
   |
LL |         Some(Wrapper::Simple::<u32>);
   |
   |
   = note: overflowed on overflowed on FingerTree<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<u32>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/dropck/dropck_no_diverge_on_nonregular_2.rs stdout ----
diff of stderr:

4 LL |     let ft =
5    |         ^^
6    |
-    = note: overflowed on FingerTree<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<i32>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
+    = note: overflowed on overflowed on FingerTree<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<i32>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
9 error: aborting due to previous error
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dropck/dropck_no_diverge_on_nonregular_2/dropck_no_diverge_on_nonregular_2.stderr
To only update this specific test, also pass `--test-args dropck/dropck_no_diverge_on_nonregular_2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dropck/dropck_no_diverge_on_nonregular_2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dropck/dropck_no_diverge_on_nonregular_2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dropck/dropck_no_diverge_on_nonregular_2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0320]: overflow while adding drop-check rules for FingerTree<i32>
   |
   |
LL |     let ft = //~ ERROR overflow while adding drop-check rules for FingerTree
   |
   |
   = note: overflowed on overflowed on FingerTree<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<i32>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/recursion/issue-38591-non-regular-dropck-recursion.rs stdout ----
diff of stderr:

4 LL | fn f(x: S<u32>) {}
6    |
6    |
-    = note: overflowed on S<fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(u32)))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))>
+    = note: overflowed on overflowed on S<fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(u32)))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))>
9 error: aborting due to previous error
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/issue-38591-non-regular-dropck-recursion/issue-38591-non-regular-dropck-recursion.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args recursion/issue-38591-non-regular-dropck-recursion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/recursion/issue-38591-non-regular-dropck-recursion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/issue-38591-non-regular-dropck-recursion" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/issue-38591-non-regular-dropck-recursion/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0320]: overflow while adding drop-check rules for S<u32>
   |
   |
LL | fn f(x: S<u32>) {} //~ ERROR overflow while adding drop-check rules for S<u32>
   |
   |
   = note: overflowed on overflowed on S<fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(fn(u32)))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))>
error: aborting due to previous error
------------------------------------------



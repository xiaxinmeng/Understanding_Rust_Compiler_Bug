plain
diff of stderr:

16   --> $DIR/issue-90170-elision-mismatch.rs:5:44
17    |
18 LL | pub fn foo2(x: &mut Vec<&'_ u8>, y: &u8) { x.push(y); }
-    |                          --         -      ^^^^^^^^^ argument requires that `'1` must outlive `'2`
-    |                          |          |
-    |                          |          let's call the lifetime of this reference `'1`
-    |                          let's call the lifetime of this reference `'2`
+    |                         -           -      ^^^^^^^^^ argument requires that `'1` must outlive `'2`
+    |                         |           |
+    |                         |           let's call the lifetime of this reference `'1`
+    |                         let's call the lifetime of this reference `'2`
24 help: consider introducing a named lifetime parameter
25    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-90170-elision-mismatch/issue-90170-elision-mismatch.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lifetimes/issue-90170-elision-mismatch.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/issue-90170-elision-mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-90170-elision-mismatch" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-90170-elision-mismatch/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | pub fn foo(x: &mut Vec<&u8>, y: &u8) { x.push(y); } //~ ERROR lifetime may not live long enough
   |                        -        -      ^^^^^^^^^ argument requires that `'1` must outlive `'2`
   |                        |        |
   |                        |        let's call the lifetime of this reference `'1`
   |                        let's call the lifetime of this reference `'2`
help: consider introducing a named lifetime parameter
   |
   |
LL | pub fn foo<'a>(x: &mut Vec<&'a u8>, y: &'a u8) { x.push(y); } //~ ERROR lifetime may not live long enough
   |           ++++              ++          ++
error: lifetime may not live long enough
  --> /checkout/src/test/ui/lifetimes/issue-90170-elision-mismatch.rs:5:44
   |
   |
LL | pub fn foo2(x: &mut Vec<&'_ u8>, y: &u8) { x.push(y); } //~ ERROR lifetime may not live long enough
   |                         -           -      ^^^^^^^^^ argument requires that `'1` must outlive `'2`
   |                         |           |
   |                         |           let's call the lifetime of this reference `'1`
   |                         let's call the lifetime of this reference `'2`
help: consider introducing a named lifetime parameter
   |
   |
LL | pub fn foo2<'a>(x: &mut Vec<&'a u8>, y: &'a u8) { x.push(y); } //~ ERROR lifetime may not live long enough
   |            ++++              ~~          ++
error: lifetime may not live long enough
  --> /checkout/src/test/ui/lifetimes/issue-90170-elision-mismatch.rs:7:63
   |
   |
LL | pub fn foo3<'a>(_other: &'a [u8], x: &mut Vec<&u8>, y: &u8) { x.push(y); } //~ ERROR lifetime may not live long enough
   |                                               -        -      ^^^^^^^^^ argument requires that `'1` must outlive `'2`
   |                                               |        |
   |                                               |        let's call the lifetime of this reference `'1`
   |                                               let's call the lifetime of this reference `'2`
help: consider introducing a named lifetime parameter
   |
   |
LL | pub fn foo3<'a>(_other: &'a [u8], x: &mut Vec<&'a u8>, y: &'a u8) { x.push(y); } //~ ERROR lifetime may not live long enough
   |                                                ++          ++
error: aborting due to 3 previous errors
------------------------------------------



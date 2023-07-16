plain

---- [ui (nll)] src/test/ui/borrowck/issue-71546.rs stdout ----
diff of stderr:

- error[E0310]: the associated type `<&'a V as IntoIterator>::Item` may not live long enough
+ error: higher-ranked lifetime error
3    |
4 LL |       let csv_str: String = value


7 LL | |         .map(|elem| elem.to_string())
9    |
9    |
-    = help: consider adding an explicit lifetime bound `<&'a V as IntoIterator>::Item: 'static`...
-    = note: ...so that the type `<&'a V as IntoIterator>::Item` will meet its required lifetime bounds...
- note: ...that is required by this bound
-   --> $DIR/issue-71546.rs:7:55
+    = note: could not prove for<'r> [closure@$DIR/issue-71546.rs:11:14: 11:37] well-formed
+ error: higher-ranked lifetime error
+   --> $DIR/issue-71546.rs:9:27
14    |
14    |
- LL |     for<'a> <&'a V as IntoIterator>::Item: ToString + 'static,
+ LL |       let csv_str: String = value
+    |  ___________________________^
+ LL | |         .into_iter()
+ LL | |         .into_iter()
+ LL | |         .map(|elem| elem.to_string())
+    |
+    |
+    = note: could not prove for<'r, 's> Map<<&'r V as IntoIterator>::IntoIter, [closure@$DIR/issue-71546.rs:11:14: 11:37]> well-formed
- error: aborting due to previous error
+ error: higher-ranked lifetime error
+   --> $DIR/issue-71546.rs:9:27
+    |
+    |
+ LL |       let csv_str: String = value
+    |  ___________________________^
+ LL | |         .into_iter()
+ LL | |         .map(|elem| elem.to_string())
+ LL | |         .collect::<String>();
+    |
+    |
+    = note: could not prove for<'r, 's> Map<<&'r V as IntoIterator>::IntoIter, [closure@$DIR/issue-71546.rs:11:14: 11:37]> well-formed
- For more information about this error, try `rustc --explain E0310`.
+ error: higher-ranked lifetime error
+   --> $DIR/issue-71546.rs:11:14
+    |
+    |
+ LL |         .map(|elem| elem.to_string())
+    |
+    |
+    = note: could not prove for<'a> <&'a V as IntoIterator>::Item: 'static
+ error: higher-ranked lifetime error
+   --> $DIR/issue-71546.rs:12:10
+    |
+    |
+ LL |         .collect::<String>();
+ 
+ error: aborting due to 5 previous errors
+ 
21 
21 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-71546.nll/issue-71546.nll.stderr
To only update this specific test, also pass `--test-args borrowck/issue-71546.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-71546.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-71546.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-71546.nll/auxiliary"
stdout: none
--- stderr -------------------------------
error: higher-ranked lifetime error
   |
   |
LL |       let csv_str: String = value //~ ERROR: the associated type `<&'a V as IntoIterator>::Item` may not live long enough
LL | |         .into_iter()
LL | |         .into_iter()
LL | |         .map(|elem| elem.to_string())
   |
   |
   = note: could not prove for<'r> [closure@/checkout/src/test/ui/borrowck/issue-71546.rs:11:14: 11:37] well-formed
error: higher-ranked lifetime error
  --> /checkout/src/test/ui/borrowck/issue-71546.rs:9:27
   |
   |
LL |       let csv_str: String = value //~ ERROR: the associated type `<&'a V as IntoIterator>::Item` may not live long enough
LL | |         .into_iter()
LL | |         .into_iter()
LL | |         .map(|elem| elem.to_string())
   |
   |
   = note: could not prove for<'r, 's> Map<<&'r V as IntoIterator>::IntoIter, [closure@/checkout/src/test/ui/borrowck/issue-71546.rs:11:14: 11:37]> well-formed
error: higher-ranked lifetime error
  --> /checkout/src/test/ui/borrowck/issue-71546.rs:9:27
   |
   |
LL |       let csv_str: String = value //~ ERROR: the associated type `<&'a V as IntoIterator>::Item` may not live long enough
LL | |         .into_iter()
LL | |         .into_iter()
LL | |         .map(|elem| elem.to_string())
LL | |         .collect::<String>();
   | |____________________________^
Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
   = note: could not prove for<'r, 's> Map<<&'r V as IntoIterator>::IntoIter, [closure@/checkout/src/test/ui/borrowck/issue-71546.rs:11:14: 11:37]> well-formed
error: higher-ranked lifetime error
  --> /checkout/src/test/ui/borrowck/issue-71546.rs:11:14
   |
   |
LL |         .map(|elem| elem.to_string())
   |
   |
   = note: could not prove for<'a> <&'a V as IntoIterator>::Item: 'static
error: higher-ranked lifetime error
  --> /checkout/src/test/ui/borrowck/issue-71546.rs:12:10
   |
   |
LL |         .collect::<String>();

error: aborting due to 5 previous errors
------------------------------------------


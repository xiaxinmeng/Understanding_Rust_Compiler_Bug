plain
diff of stderr:

2   --> $DIR/doc-cfg.rs:3:7
3    |
4 LL | #[doc(cfg(), cfg(foo, bar))]
-    |       ^^^^^
+    |       ^^^^^ help: expected syntax is: `cfg(/* predicate */)`
6 
7 error: multiple `cfg` predicates are specified

14   --> $DIR/doc-cfg.rs:7:7
15    |
15    |
16 LL | #[doc(cfg())]
-    |       ^^^^^
+    |       ^^^^^ help: expected syntax is: `cfg(/* predicate */)`
18 
19 error: multiple `cfg` predicates are specified


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-cfg/doc-cfg.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-cfg/doc-cfg.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args doc-cfg.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/doc-cfg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-cfg" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-cfg/auxiliary"
stdout: none
--- stderr -------------------------------
error: `cfg` predicate is not specified
   |
   |
LL | #[doc(cfg(), cfg(foo, bar))]
   |       ^^^^^ help: expected syntax is: `cfg(/* predicate */)`

error: multiple `cfg` predicates are specified
   |
   |
LL | #[doc(cfg(), cfg(foo, bar))]

error: `cfg` predicate is not specified
  --> /checkout/src/test/rustdoc-ui/doc-cfg.rs:7:7
   |
   |
LL | #[doc(cfg())] //~ ERROR
   |       ^^^^^ help: expected syntax is: `cfg(/* predicate */)`

error: multiple `cfg` predicates are specified
   |
   |
LL | #[doc(cfg(foo, bar))] //~ ERROR

error: aborting due to 4 previous errors
------------------------------------------


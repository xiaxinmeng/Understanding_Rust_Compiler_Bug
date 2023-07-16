plain
Some tests failed in compiletest suite=rustdoc-ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
..
failures:

---- [ui] src/test/rustdoc-ui/diagnostic-width.rs stdout ----


- error: this URL is not a hyperlink
-   --> $DIR/diagnostic-width.rs:4:41
-    |
- LL | ... a http://link.com
-    |       ^^^^^^^^^^^^^^^ help: use an automatic link instead: `<http://link.com>`
- note: the lint level is defined here
-   --> $DIR/diagnostic-width.rs:2:9
-    |
-    |
- LL | ...ny(rustdoc::bare_url...
-    |       ^^^^^^^^^^^^^^^^^^
-    = note: bare URLs are not automatically turned into clickable links
- error: aborting due to previous error
- error: aborting due to previous error
+ error: the `-Z unstable-options` flag must also be passed to enable the flag `diagnostic-width`
16 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/diagnostic-width/diagnostic-width.stderr
To only update this specific test, also pass `--test-args diagnostic-width.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/diagnostic-width.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/diagnostic-width" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--diagnostic-width=10" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/diagnostic-width/auxiliary"
stdout: none
--- stderr -------------------------------
error: the `-Z unstable-options` flag must also be passed to enable the flag `diagnostic-width`



failures:

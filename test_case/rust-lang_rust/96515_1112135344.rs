plain
Successfully built 45af15d89357
Successfully tagged rust-ci:latest
Built container sha256:45af15d89357899e3ca58844c8e80cd8f9e7b2477e1accec07cf66d5fea242ae
Uploading finished image to https://ci-caches.rust-lang.org/docker/7a933025cb171c4b5f5b6011a3583795b8dcc545c75914dd6be8b89a1706a0990abf8b940dcd0d758184ddd671b7cd225927f49feaa72b37c79d66fed681b775
upload failed: - to s3://rust-lang-ci-sccache2/docker/7a933025cb171c4b5f5b6011a3583795b8dcc545c75914dd6be8b89a1706a0990abf8b940dcd0d758184ddd671b7cd225927f49feaa72b37c79d66fed681b775 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-12]
---
........................................................................................ 6512/13030
.................................................................i...................... 6600/13030
...............................ii.ii........i...i....................................... 6688/13030
........................................................................................ 6776/13030
.............................................................i....i...F................. 6864/13030
.................i...................................................................... 7040/13030
.................................i...................................................... 7128/13030
........................................................................................ 7216/13030
........................................................................................ 7304/13030
---
........................................................................................ 13024/13030
......
failures:

---- [ui] src/test/ui/lifetimes/re-empty-in-error.rs stdout ----


1 error[E0477]: the type `&'b ()` does not fulfill the required lifetime
-   --> $DIR/re-empty-in-error.rs:9:5
+   --> $DIR/re-empty-in-error.rs:8:5
4 LL |     foo(&10);
5    |     ^^^

6    |
6    |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
7 note: type must outlive the empty lifetime as required by this binding
-   --> $DIR/re-empty-in-error.rs:4:47
+   --> $DIR/re-empty-in-error.rs:3:47
9    |
10 LL | fn foo<'a>(_a: &'a u32) where for<'b> &'b (): 'a {


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/re-empty-in-error/re-empty-in-error.stderr
To only update this specific test, also pass `--test-args lifetimes/re-empty-in-error.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/re-empty-in-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/re-empty-in-error" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/re-empty-in-error/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0477]: the type `&'b ()` does not fulfill the required lifetime
  --> /checkout/src/test/ui/lifetimes/re-empty-in-error.rs:8:5
LL |     foo(&10);
   |     ^^^
   |
note: type must outlive the empty lifetime as required by this binding
note: type must outlive the empty lifetime as required by this binding
  --> /checkout/src/test/ui/lifetimes/re-empty-in-error.rs:3:47
   |
LL | fn foo<'a>(_a: &'a u32) where for<'b> &'b (): 'a {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0477`.

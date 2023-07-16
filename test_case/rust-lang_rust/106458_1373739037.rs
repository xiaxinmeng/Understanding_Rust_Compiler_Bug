plain
Successfully built 64f79a117770
Successfully tagged rust-ci:latest
Built container sha256:64f79a1177703d9727c19b67d1e7db7d50f09973df15f08488850ca6e67cd0b6
Uploading finished image to https://ci-caches.rust-lang.org/docker/33dce652da939f1fdd21916653a8d85737ef5d29f08ef2b4d13fbcd7ac37c8f60ac9d9ffb0ec3dd239bf605394bb75308276232793206db1f0433dec328640fe
upload failed: - to s3://rust-lang-ci-sccache2/docker/33dce652da939f1fdd21916653a8d85737ef5d29f08ef2b4d13fbcd7ac37c8f60ac9d9ffb0ec3dd239bf605394bb75308276232793206db1f0433dec328640fe Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-13]
---
 - Checking "/checkout/src/librustdoc/html/static/css/themes/ayu.css"... OK
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 197 tests
...........F....F..FF.F.F................................i.............................. 88/197
Some tests failed in compiletest suite=rustdoc-ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.....................
failures:


---- [ui] checkout/tests/rustdoc-ui/coverage/basic.rs stdout ----


1 +-------------------------------------+------------+------------+------------+------------+
2 | File                                | Documented | Percentage |   Examples | Percentage |
3 +-------------------------------------+------------+------------+------------+------------+
- | ...est/rustdoc-ui/coverage/basic.rs |          7 |      50.0% |          0 |       0.0% |
+ | ...sts/rustdoc-ui/coverage/basic.rs |          7 |      50.0% |          0 |       0.0% |
5 +-------------------------------------+------------+------------+------------+------------+
6 | Total                               |          7 |      50.0% |          0 |       0.0% |
7 +-------------------------------------+------------+------------+------------+------------+

The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/basic/basic.stdout
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args coverage/basic.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/coverage/basic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/basic" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/basic/auxiliary" "-Z" "unstable-options" "--show-coverage"
--- stdout -------------------------------
+-------------------------------------+------------+------------+------------+------------+
| File                                | Documented | Percentage |   Examples | Percentage |
+-------------------------------------+------------+------------+------------+------------+
| ...sts/rustdoc-ui/coverage/basic.rs |          7 |      50.0% |          0 |       0.0% |
+-------------------------------------+------------+------------+------------+------------+
| Total                               |          7 |      50.0% |          0 |       0.0% |
+-------------------------------------+------------+------------+------------+------------+
stderr: none



---- [ui] checkout/tests/rustdoc-ui/coverage/empty.rs stdout ----


1 +-------------------------------------+------------+------------+------------+------------+
2 | File                                | Documented | Percentage |   Examples | Percentage |
3 +-------------------------------------+------------+------------+------------+------------+
- | ...est/rustdoc-ui/coverage/empty.rs |          0 |       0.0% |          0 |       0.0% |
+ | ...sts/rustdoc-ui/coverage/empty.rs |          0 |       0.0% |          0 |       0.0% |
5 +-------------------------------------+------------+------------+------------+------------+
6 | Total                               |          0 |       0.0% |          0 |       0.0% |
7 +-------------------------------------+------------+------------+------------+------------+

The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/empty/empty.stdout
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args coverage/empty.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/coverage/empty.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/empty" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/empty/auxiliary" "-Z" "unstable-options" "--show-coverage"
--- stdout -------------------------------
+-------------------------------------+------------+------------+------------+------------+
| File                                | Documented | Percentage |   Examples | Percentage |
+-------------------------------------+------------+------------+------------+------------+
| ...sts/rustdoc-ui/coverage/empty.rs |          0 |       0.0% |          0 |       0.0% |
+-------------------------------------+------------+------------+------------+------------+
| Total                               |          0 |       0.0% |          0 |       0.0% |
+-------------------------------------+------------+------------+------------+------------+
stderr: none



---- [ui] checkout/tests/rustdoc-ui/coverage/exotic.rs stdout ----


1 +-------------------------------------+------------+------------+------------+------------+
2 | File                                | Documented | Percentage |   Examples | Percentage |
3 +-------------------------------------+------------+------------+------------+------------+
- | ...st/rustdoc-ui/coverage/exotic.rs |          3 |     100.0% |          0 |       0.0% |
+ | ...ts/rustdoc-ui/coverage/exotic.rs |          3 |     100.0% |          0 |       0.0% |
5 +-------------------------------------+------------+------------+------------+------------+
6 | Total                               |          3 |     100.0% |          0 |       0.0% |
7 +-------------------------------------+------------+------------+------------+------------+

The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/exotic/exotic.stdout
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args coverage/exotic.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/coverage/exotic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/exotic" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/exotic/auxiliary" "-Z" "unstable-options" "--show-coverage"
--- stdout -------------------------------
+-------------------------------------+------------+------------+------------+------------+
| File                                | Documented | Percentage |   Examples | Percentage |
+-------------------------------------+------------+------------+------------+------------+
| ...ts/rustdoc-ui/coverage/exotic.rs |          3 |     100.0% |          0 |       0.0% |
+-------------------------------------+------------+------------+------------+------------+
| Total                               |          3 |     100.0% |          0 |       0.0% |
+-------------------------------------+------------+------------+------------+------------+
stderr: none



---- [ui] checkout/tests/rustdoc-ui/coverage/enums.rs stdout ----


1 +-------------------------------------+------------+------------+------------+------------+
2 | File                                | Documented | Percentage |   Examples | Percentage |
3 +-------------------------------------+------------+------------+------------+------------+
- | ...est/rustdoc-ui/coverage/enums.rs |          6 |      75.0% |          0 |       0.0% |
+ | ...sts/rustdoc-ui/coverage/enums.rs |          6 |      75.0% |          0 |       0.0% |
5 +-------------------------------------+------------+------------+------------+------------+
6 | Total                               |          6 |      75.0% |          0 |       0.0% |
7 +-------------------------------------+------------+------------+------------+------------+

The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/enums/enums.stdout
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args coverage/enums.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/coverage/enums.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/enums" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/enums/auxiliary" "-Z" "unstable-options" "--show-coverage"
--- stdout -------------------------------
+-------------------------------------+------------+------------+------------+------------+
| File                                | Documented | Percentage |   Examples | Percentage |
+-------------------------------------+------------+------------+------------+------------+
| ...sts/rustdoc-ui/coverage/enums.rs |          6 |      75.0% |          0 |       0.0% |
+-------------------------------------+------------+------------+------------+------------+
| Total                               |          6 |      75.0% |          0 |       0.0% |
+-------------------------------------+------------+------------+------------+------------+
stderr: none



---- [ui] checkout/tests/rustdoc-ui/coverage/private.rs stdout ----


1 +-------------------------------------+------------+------------+------------+------------+
2 | File                                | Documented | Percentage |   Examples | Percentage |
3 +-------------------------------------+------------+------------+------------+------------+
- | ...t/rustdoc-ui/coverage/private.rs |          4 |      57.1% |          0 |       0.0% |
+ | ...s/rustdoc-ui/coverage/private.rs |          4 |      57.1% |          0 |       0.0% |
5 +-------------------------------------+------------+------------+------------+------------+
6 | Total                               |          4 |      57.1% |          0 |       0.0% |
7 +-------------------------------------+------------+------------+------------+------------+

The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/private/private.stdout
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args coverage/private.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/coverage/private.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/private" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/private/auxiliary" "-Z" "unstable-options" "--show-coverage" "--document-private-items"
--- stdout -------------------------------
+-------------------------------------+------------+------------+------------+------------+
| File                                | Documented | Percentage |   Examples | Percentage |
+-------------------------------------+------------+------------+------------+------------+
| ...s/rustdoc-ui/coverage/private.rs |          4 |      57.1% |          0 |       0.0% |
+-------------------------------------+------------+------------+------------+------------+
| Total                               |          4 |      57.1% |          0 |       0.0% |
+-------------------------------------+------------+------------+------------+------------+
stderr: none



---- [ui] checkout/tests/rustdoc-ui/coverage/traits.rs stdout ----


1 +-------------------------------------+------------+------------+------------+------------+
2 | File                                | Documented | Percentage |   Examples | Percentage |
3 +-------------------------------------+------------+------------+------------+------------+
- | ...st/rustdoc-ui/coverage/traits.rs |          8 |      88.9% |          0 |       0.0% |
+ | ...ts/rustdoc-ui/coverage/traits.rs |          8 |      88.9% |          0 |       0.0% |
5 +-------------------------------------+------------+------------+------------+------------+
6 | Total                               |          8 |      88.9% |          0 |       0.0% |
7 +-------------------------------------+------------+------------+------------+------------+

The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/traits/traits.stdout
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args coverage/traits.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/coverage/traits.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/traits" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/coverage/traits/auxiliary" "-Z" "unstable-options" "--show-coverage"
--- stdout -------------------------------
+-------------------------------------+------------+------------+------------+------------+
| File                                | Documented | Percentage |   Examples | Percentage |
+-------------------------------------+------------+------------+------------+------------+
| ...ts/rustdoc-ui/coverage/traits.rs |          8 |      88.9% |          0 |       0.0% |
+-------------------------------------+------------+------------+------------+------------+
| Total                               |          8 |      88.9% |          0 |       0.0% |
+-------------------------------------+------------+------------+------------+------------+
stderr: none




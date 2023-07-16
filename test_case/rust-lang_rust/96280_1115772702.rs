plain
To only update this specific test, also pass `--test-args block-doc-comment.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/block-doc-comment.rs" "-Zthreads=1" "--target=aarch64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/rustdoc-ui/block-doc-comment" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/rustdoc-ui/block-doc-comment/auxiliary"
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
------------------------------------------

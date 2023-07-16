plain
........................................................................................ 13288/13301
.............
failures:

---- [ui] src/test/ui/traits/issue-97695-double-trivial-bound.rs stdout ----
error: ui test compiled successfully!
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/issue-97695-double-trivial-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-97695-double-trivial-bound" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zinline-mir" "--emit=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-97695-double-trivial-bound/auxiliary"
stdout: none
stderr: none


failures:
    [ui] src/test/ui/traits/issue-97695-double-trivial-bound.rs

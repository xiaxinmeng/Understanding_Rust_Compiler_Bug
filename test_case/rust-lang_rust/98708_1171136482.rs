
---- [ui] src/test/rustdoc-ui/issue-98690.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/home/foobar/gg/rust/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/home/foobar/gg/rust/src/test/rustdoc-ui/issue-foo.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/home/foobar/gg/rust/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-foo" "-Cdebuginfo=0" "-Lnative=/home/foobar/gg/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "--persist-doctests" "/../../" "-Z" "unstable-options" "-L" "/home/foobar/gg/rust/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-foo/auxiliary"
stdout: none
--- stderr -------------------------------
Couldn't create directory for doctest executables: Permission denied (os error 13)
------------------------------------------

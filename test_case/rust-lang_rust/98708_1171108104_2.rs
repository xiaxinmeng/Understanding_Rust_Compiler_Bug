
---- [rustdoc] src/test/rustdoc/issue-98690.rs stdout ----

error: rustdoc failed!
status: exit status: 1
command: "/home/foobar/gg/rust/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/home/foobar/gg/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/home/foobar/gg/rust/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-foo/auxiliary" "-o" "/home/foobar/gg/rust/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-foo" "--deny" "warnings" "/home/foobar/gg/rust/src/test/rustdoc/issue-foo.rs" "--persist-doctests" "/cannot_write_here" "-Z" "unstable-options" "-hfff"
stdout: none
--- stderr -------------------------------
error: Unrecognized option: 'f'
------------------------------------------

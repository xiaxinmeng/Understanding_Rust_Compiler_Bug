
[00:42:51] error: the option `Z` is only accepted on the nightly compiler
[00:42:51] 
[00:42:51] thread '<unnamed>' panicked at 'Box<Any>', /checkout/src/librustc/session/mod.rs:930:4
[00:42:51] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:42:51] 
[00:42:51] 
[00:42:51] command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "--html-after-content" "/checkout/src/doc/footer.inc" "--html-before-content" "/checkout/obj/build/powerpc64-unknown-linux-gnu/doc/version_info.html" "--html-in-header" "/checkout/src/doc/favicon.inc" "--markdown-playground-url" "https://play.rust-lang.org/" "-o" "/checkout/obj/build/powerpc64-unknown-linux-gnu/doc" "/checkout/src/doc/guide-testing.md" "--markdown-css" "rust.css"
[00:42:51] expected success, got: exit code: 101
[00:42:51] 
[00:42:51] 
[00:42:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host powerpc64-unknown-linux-gnu --target powerpc64-unknown-linux-gnu
[00:42:51] Build completed unsuccessfully in 0:40:44

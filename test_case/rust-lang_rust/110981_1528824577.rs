plain
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 222 tests
....................................i...................................................  88/222
......................................................2023-04-29T16:29:16.918716Z ERROR compiletest::runtest: fatal error, panic: "aux-build `/checkout/tests/rustdoc-ui/issues/auxiliary/panic-handler.rs` source not found"
F................................. 176/222

failures:

---- [ui] tests/rustdoc-ui/issues/issue-107918.rs stdout ----
---- [ui] tests/rustdoc-ui/issues/issue-107918.rs stdout ----

error: aux-build `/checkout/tests/rustdoc-ui/issues/auxiliary/panic-handler.rs` source not found
thread '[ui] tests/rustdoc-ui/issues/issue-107918.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2287:9


failures:
    [ui] tests/rustdoc-ui/issues/issue-107918.rs

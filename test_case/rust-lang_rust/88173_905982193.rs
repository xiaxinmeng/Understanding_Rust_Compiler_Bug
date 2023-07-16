plain
    Finished release [optimized] target(s) in 25.17s
     Running unittests (build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/rustdoc-4c36efc7f94f9616)

running 74 tests
..........................................F.F.............................


---- html::length_limit::tests::past_the_limit stdout ----
thread 'html::length_limit::tests::past_the_limit' panicked at 'assertion failed: `(left == right)`
  left: `"<p><strong>word#0</strong><strong>word#1</strong><strong>word#2</strong></p>"`,
 right: `"<p><strong>word#1</strong><strong>word#2</strong><strong>word#3</strong></p>"`', src/librustdoc/html/length_limit/tests.rs:89:5

---- html::length_limit::tests::limit_0 stdout ----
thread 'html::length_limit::tests::limit_0' panicked at 'called `Option::unwrap()` on a `None` value', src/librustdoc/html/length_limit.rs:89:49

failures:
    html::length_limit::tests::limit_0
    html::length_limit::tests::limit_0
    html::length_limit::tests::past_the_limit
test result: FAILED. 72 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


error: test failed, to rerun pass '-p rustdoc --lib'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"


Build completed unsuccessfully in 0:25:16

` is wrong.

> [#[ignore]](https://doc.rust-lang.org/reference/attributes/testing.html#the-ignore-attribute) — Indicates that the test function will be compiled, but not run ***by default*** [emphasis mine]. See the [--ignored](https://doc.rust-lang.org/rustc/tests/index.html#--ignored) and [--include-ignored](https://doc.rust-lang.org/rustc/tests/index.html#--include-ignored) options to run these tests. [[rustc book §6 Tests, Test attributes](https://doc.rust-lang.org/rustc/tests/index.html#test-attributes)]

`#[ignore]`, and thus by extension 
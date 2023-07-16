plain
Successfully built 6b849e2e31e4
Successfully tagged rust-ci:latest
Built container sha256:6b849e2e31e4b5dcf1814265904ec42cdf17d9836fbfe4e21613ddee231babaf
Uploading finished image to https://ci-caches.rust-lang.org/docker/b8522bdbde0170e6b8638c2ee7936d2221f0e051b4634ed61ad299a1775319b857fabeb7dc5d6fa62739b362a15aa7c74c4f64f6f12f1718e9d3e82ccdcf01ba
upload failed: - to s3://rust-lang-ci-sccache2/docker/b8522bdbde0170e6b8638c2ee7936d2221f0e051b4634ed61ad299a1775319b857fabeb7dc5d6fa62739b362a15aa7c74c4f64f6f12f1718e9d3e82ccdcf01ba Unable to locate credentials
 * branch              master     -> FETCH_HEAD
[CI_JOB_NAME=x86_64-gnu-llvm-10]
[CI_JOB_NAME=x86_64-gnu-llvm-10]
---
doc tests for: /checkout/src/doc/unstable-book/src/language-features/plugin.md
doc tests for: /checkout/src/doc/unstable-book/src/language-features/raw-dylib.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/src/doc/unstable-book/src/language-features/raw-dylib.md" "--test-args" ""

stdout ----

running 1 test
running 1 test
test /checkout/src/doc/unstable-book/src/language-features/raw-dylib.md - The_tracking_issue_for_this_feature_is__ (line 12) ... FAILED

failures:

---- /checkout/src/doc/unstable-book/src/language-features/raw-dylib.md - The_tracking_issue_for_this_feature_is__ (line 12) stdout ----
warning: the feature `raw_dylib` is incomplete and may not be safe to use and/or cause compiler crashes
 --> /checkout/src/doc/unstable-book/src/language-features/raw-dylib.md:13:12
  |
2 | #![feature(raw_dylib)]
  |
  = note: `#[warn(incomplete_features)]` on by default
  = note: see issue #58713 <https://github.com/rust-lang/rust/issues/58713> for more information


error: `#[link(...)]` with `kind = "raw-dylib"` only supported on Windows
 --> /checkout/src/doc/unstable-book/src/language-features/raw-dylib.md:15:1
  |
4 | #[link(name="library", kind="raw-dylib")]

error: aborting due to previous error; 1 warning emitted

Couldn't compile the test.


[ERROR] failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
Caused by:
  patch for `bar` in `https://github.com/rust-lang/crates.io-index` failed to resolve
Caused by:
  The patch location `[..]/foo/bar` contains a `bar` package with version `0.1.0`, 
  but the patch definition requires `^0.1.1`.
  Check that the version in the patch location is what you expect, and update the 
  patch definition to match.

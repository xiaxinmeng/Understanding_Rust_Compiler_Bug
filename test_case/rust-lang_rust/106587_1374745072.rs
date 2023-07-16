plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
...................i.................................................................... 3960/3987
...........................
failures:

---- src/pin.rs - pin::Pin<P>::new_unchecked (line 626) stdout ----
error: the feature `pin_macro` has been stable since 1.67.0-beta.3 and no longer requires an attribute to enable
  |
  |
3 | #![feature(pin_macro)]
  |
note: the lint level is defined here
 --> src/pin.rs:624:9
  |
---
    src/pin.rs - pin::Pin<P>::new_unchecked (line 626)

test result: FAILED. 3950 passed; 1 failed; 36 ignored; 0 measured; 0 filtered out; finished in 52.07s

error: doctest failed, to rerun pass `-p core --doc`

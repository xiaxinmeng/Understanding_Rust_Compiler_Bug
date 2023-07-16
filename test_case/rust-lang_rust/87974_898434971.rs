plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking rand v0.7.3
    Checking std v0.0.0 (/checkout/library/std)
    Checking core v0.0.0 (/checkout/library/core)
    Checking alloc v0.0.0 (/checkout/library/alloc)
error[E0432]: unresolved import `assert_precise_size_hints`
     |
1037 |             use assert_precise_size_hints as a;
1037 |             use assert_precise_size_hints as a;
     |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no external crate `assert_precise_size_hints`
For more information about this error, try `rustc --explain E0432`.
error: could not compile `alloc` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed

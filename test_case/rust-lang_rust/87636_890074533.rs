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
    Checking core v0.0.0 (/checkout/library/core)
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking std v0.0.0 (/checkout/library/std)
error[E0658]: use of unstable library feature 'unzip_option': recently added
    |
    |
427 |     assert_eq!(x.unzip(), (Some(10), Some("foo")));
    |
    |
    = help: add `#![feature(unzip_option)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unzip_option': recently added
    |
    |
428 |     assert_eq!(y.unzip(), (None, None));
    |
    |
    = help: add `#![feature(unzip_option)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unzip_option': recently added
    |
    |
439 |     let a = z.unzip();
    |
    |
    = help: add `#![feature(unzip_option)]` to the crate attributes to enable
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0658`.
error: could not compile `core`

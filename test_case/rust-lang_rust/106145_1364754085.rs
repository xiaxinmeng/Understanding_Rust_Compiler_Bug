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
    Checking rand v0.7.3
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking std v0.0.0 (/checkout/library/std)
    Checking core v0.0.0 (/checkout/library/core)
error[E0599]: the function or associated item `default` exists for array `[DoesNotImplDefault; 0]`, but its trait bounds were not satisfied
    |
    |
303 |     struct DoesNotImplDefault;
    |     ------------------------- doesn't satisfy `DoesNotImplDefault: Default`
304 |
305 |     let _arr = <[DoesNotImplDefault; 0]>::default();
    |                                           ^^^^^^^ function or associated item cannot be called on `[DoesNotImplDefault; 0]` due to unsatisfied trait bounds
    = note: the following trait bounds were not satisfied:
    = note: the following trait bounds were not satisfied:
            `DoesNotImplDefault: Default`
            which is required by `[DoesNotImplDefault; 0]: Default`
help: consider annotating `DoesNotImplDefault` with `#[derive(Default)]`
303 |     #[derive(Default)]
    |

For more information about this error, try `rustc --explain E0599`.

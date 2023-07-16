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
   Compiling compiletest v0.0.0 (/checkout/src/tools/compiletest)
error[E0308]: mismatched types
   --> src/tools/compiletest/src/main.rs:804:38
    |
804 |     test::DynTestFn(Box::new(move || runtest::run(config, &testpaths, revision.as_deref())))
    |
    = note:   expected enum `std::result::Result<(), std::string::String>`
            found unit type `()`
            found unit type `()`
help: try wrapping the expression in `json::_::_serde::__private::Ok`
    |
804 |     test::DynTestFn(Box::new(move || json::_::_serde::__private::Ok(runtest::run(config, &testpaths, revision.as_deref()))))

For more information about this error, try `rustc --explain E0308`.
error: could not compile `compiletest` due to previous error
Build completed unsuccessfully in 0:01:04

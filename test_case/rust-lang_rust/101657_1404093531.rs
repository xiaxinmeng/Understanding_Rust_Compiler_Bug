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
    Checking rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
    Checking rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
error: `#[suggestion_verbose(...)]` is not a valid attribute
    |
    |
320 |     #[suggestion_verbose(code = "fn ", applicability = "maybe-incorrect")]
    |
    |
    = help: Use `#[suggestion(..., style = "verbose")]` instead

error: `#[suggestion_verbose(...)]` is not a valid attribute
    |
    |
407 |     #[suggestion_verbose(code = "{correct_order}", applicability = "machine-applicable")]
    |
    |
    = help: Use `#[suggestion(..., style = "verbose")]` instead
error: could not compile `rustc_ast_passes` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_ast_passes` due to 2 previous errors
Build completed unsuccessfully in 0:01:05

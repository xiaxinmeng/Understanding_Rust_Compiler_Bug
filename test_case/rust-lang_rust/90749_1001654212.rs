plain
    Checking tempfile v3.2.0
    Checking synstructure v0.12.6
    Checking rustc_macros v0.1.0 (/checkout/compiler/rustc_macros)
   Compiling tracing-attributes v0.1.17
   Compiling chalk-derive v0.76.0-dev.0 (https://github.com/rust-lang/chalk?branch=master#f518c7c7)
    Checking chalk-ir v0.76.0-dev.0 (https://github.com/rust-lang/chalk?branch=master#f518c7c7)
    Checking tracing-subscriber v0.3.3
    Checking rustc_index v0.0.0 (/checkout/compiler/rustc_index)
    Checking rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
    Checking tracing-tree v0.2.0
    Checking tracing-tree v0.2.0
    Checking chalk-solve v0.76.0-dev.0 (https://github.com/rust-lang/chalk?branch=master#f518c7c7)
    Checking rustc_type_ir v0.0.0 (/checkout/compiler/rustc_type_ir)
    Checking rustc_span v0.0.0 (/checkout/compiler/rustc_span)
    Checking rustc_ast v0.0.0 (/checkout/compiler/rustc_ast)
    Checking rustc_target v0.0.0 (/checkout/compiler/rustc_target)
    Checking rustc_target v0.0.0 (/checkout/compiler/rustc_target)
    Checking rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
    Checking rustc_parse_format v0.0.0 (/checkout/compiler/rustc_parse_format)
    Checking chalk-engine v0.76.0-dev.0 (https://github.com/rust-lang/chalk?branch=master#f518c7c7)
    Checking rls-span v0.5.3
    Checking rls-data v0.19.1
    Checking rustc_hir v0.0.0 (/checkout/compiler/rustc_hir)
    Checking rustc_lint_defs v0.0.0 (/checkout/compiler/rustc_lint_defs)
---
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking measureme v10.0.0
    Checking synstructure v0.12.6
    Checking rustc_macros v0.1.0 (/checkout/compiler/rustc_macros)
   Compiling tracing-attributes v0.1.17
   Compiling chalk-derive v0.76.0-dev.0 (https://github.com/rust-lang/chalk?branch=master#f518c7c7)
    Checking chalk-ir v0.76.0-dev.0 (https://github.com/rust-lang/chalk?branch=master#f518c7c7)
    Checking tracing-subscriber v0.3.3
    Checking rustc_index v0.0.0 (/checkout/compiler/rustc_index)
    Checking rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
    Checking tracing-tree v0.2.0
    Checking tracing-tree v0.2.0
    Checking chalk-solve v0.76.0-dev.0 (https://github.com/rust-lang/chalk?branch=master#f518c7c7)
    Checking rustc_type_ir v0.0.0 (/checkout/compiler/rustc_type_ir)
    Checking rustc_span v0.0.0 (/checkout/compiler/rustc_span)
    Checking rustc_ast v0.0.0 (/checkout/compiler/rustc_ast)
    Checking rustc_target v0.0.0 (/checkout/compiler/rustc_target)
    Checking rustc_target v0.0.0 (/checkout/compiler/rustc_target)
    Checking rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
    Checking rustc_parse_format v0.0.0 (/checkout/compiler/rustc_parse_format)
    Checking chalk-engine v0.76.0-dev.0 (https://github.com/rust-lang/chalk?branch=master#f518c7c7)
    Checking rls-span v0.5.3
    Checking rls-data v0.19.1
    Checking rustc_lint_defs v0.0.0 (/checkout/compiler/rustc_lint_defs)
    Checking rustc_hir v0.0.0 (/checkout/compiler/rustc_hir)
---
   Compiling regex v1.5.4
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 9.69s
tidy check
tidy error: invalid source: "git+https://github.com/rust-lang/chalk?branch=master#f518c7c7ed723dd14486bc966e2474e46fe04269"
tidy error: invalid source: "git+https://github.com/rust-lang/chalk?branch=master#f518c7c7ed723dd14486bc966e2474e46fe04269"
tidy error: invalid source: "git+https://github.com/rust-lang/chalk?branch=master#f518c7c7ed723dd14486bc966e2474e46fe04269"
tidy error: invalid source: "git+https://github.com/rust-lang/chalk?branch=master#f518c7c7ed723dd14486bc966e2474e46fe04269"
Checking which error codes lack tests...
* 628 error codes
* highest error code: E0786
Found 502 error codes
Found 502 error codes
Found 0 error codes with no tests
Done!
* 361 features
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:12

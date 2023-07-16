toml
[files]
extend-exclude = [
    "*.json",
    "*.sh",
    "**/benches/**",
    "**/*test*/**",
    "**/*test*.rs",
    "RELEASES.md",
    "**/patches/**",
    "**/archs.rs",
    "compiler/rustc_codegen_cranelift/example/**",
    "library/core/src/unicode/unicode_data.rs",
    "src/ci/docker/**",

    ### manually vetted files
    "src/doc/unstable-book/src/compiler-flags/sanitizer.md",
    "src/doc/rustdoc/src/what-is-rustdoc.md",
    "src/tools/lint-docs/src/lib.rs",
    "library/test/src/term/terminfo/parser/compiled.rs",
    # typos already present in the `winerror.h` header file
    "library/std/src/sys/windows/c/errors.rs",
    # intentional `widnows` typo
    "compiler/rustc_lint_defs/src/builtin.rs",
    # git commit sha
    "compiler/rustc_interface/src/util.rs",

    ### [rustfmt.toml].exclude
    "/build/",
    "/*-build/",
    "/build-*/",
    "/vendor/",

    # tests for now are not formatted, as they are sometimes pretty-printing constrained
    # (and generally rustfmt can move around comments in UI-testing incompatible ways)
    "tests",

    # do not format submodules
    "library/backtrace",
    "library/portable-simd",
    "library/stdarch",
    "compiler/rustc_codegen_gcc",
    "src/doc/book",
    "src/doc/edition-guide",
    "src/doc/embedded-book",
    "src/doc/nomicon",
    "src/doc/reference",
    "src/doc/rust-by-example",
    "src/doc/rustc-dev-guide",
    "src/llvm-project",
    "src/tools/cargo",
    "src/tools/clippy",
    "src/tools/miri",
    "src/tools/rls",
    "src/tools/rust-analyzer",
    "src/tools/rustfmt",
    "src/tools/rust-installer",

    # these are ignored by a standard cargo fmt run
    "compiler/rustc_codegen_cranelift/y.rs", # running rustfmt breaks this file
    "compiler/rustc_codegen_cranelift/example",
    "compiler/rustc_codegen_cranelift/scripts",
]
ignore-parent = true

[default]
binary = false
locale = "en"
extend-ignore-identifiers-re = [
    # workaround for no way to specify regex flags
    "[nN][tT][oO]",
    "[nN][uU][mM][eE][rR]",
    "[bB][aA][rR][eE]",
    "[pP][rR][eE][vV]",
    "[rR][eE][tT]",
    "[gG][oO][eE][sS]",
    "[oO][tT]",
    "[iI][nN][oO][uU][tT]",
    "[nN][dD]",
    "[tT][hH][nN]",
    "[kK][eE][tT]",
    "[mM][iI][sS]",
    "[oO][pP][tT][iI][nN]",
    "[rR][eE][lL][aA]",
    "[sS][eE][hH]",
    "[wW][tT][hH]",
    "[tT][pP][yY][oO]",
    "[lL][iI][tT][sS]?",
    "RealOLT",
    "OLT",
    "LT",
    "VALUEs",
    "tolen",
    "makro",
    # library/alloc/src/collections/vec_deque/**
    "deques",
    # library/core/src/str/mod.rs
    "pard",
    # https://docs.rs/wasi/latest/wasi/constant.ERRNO_ACCES.html
    "ERRNO_ACCES",
    # /usr/x86_64-w64-mingw32/include/winerror.h
    "ERROR_FILENAME_EXCED_RANGE",
    # drain_unstalled_obligations
    "unstalled",
    "ba",
    "fpr",
    "[fF][oO]",
]

[default.extend-identifiers]
MisMatch = "Mismatch"
FieldMisMatch = "FieldMismatch"

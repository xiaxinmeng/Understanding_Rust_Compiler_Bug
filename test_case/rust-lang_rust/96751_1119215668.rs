plain
    Checking petgraph v0.5.1
    Checking gimli v0.26.1
    Checking object v0.28.1
    Checking tempfile v3.2.0
    Checking indexmap v2.0.0-pre (https://github.com/cuviper/indexmap?branch=indexable#3b6d807f)
    Checking synstructure v0.12.6
    Checking rustc_macros v0.1.0 (/checkout/compiler/rustc_macros)
   Compiling tracing-attributes v0.1.18
   Compiling unic-langid-macros-impl v0.9.0
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
    Checking unic-ucd-version v0.9.0
   Compiling generic-array v0.14.4
   Compiling ahash v0.7.4
   Compiling unic-langid-impl v0.9.0
    Checking indexmap v2.0.0-pre (https://github.com/cuviper/indexmap?branch=indexable#3b6d807f)
    Checking thread_local v1.1.4
    Checking miniz_oxide v0.4.0
    Checking unicode-normalization v0.1.13
    Checking expect-test v1.0.1
---
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 6.94s
tidy check
tidy: Skipping binary file check, read-only filesystem
tidy error: invalid source: "git+https://github.com/cuviper/indexmap?branch=indexable#3b6d807f460308e6de30cd34f9516aca624615f2"
* 629 error codes
* highest error code: E0787
Found 504 error codes
Found 0 error(s) in error codes

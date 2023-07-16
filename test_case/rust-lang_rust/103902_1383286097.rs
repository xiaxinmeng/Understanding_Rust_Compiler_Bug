plain
Successfully built 3934aa007fbd
Successfully tagged rust-ci:latest
Built container sha256:3934aa007fbd4eb86b50399393f40a51af5e1c90b6d138723e4e4995233235bc
Uploading finished image to https://ci-caches.rust-lang.org/docker/d33e4deadd55ac310bf6d14046967e2ae76df1d0cbcfb3cbdfd57a515110e5c43bea0fc68be40129a2ca62a8c3c0cddee210b82b000006fe1b3d251a4634baf3
upload failed: - to s3://rust-lang-ci-sccache2/docker/d33e4deadd55ac310bf6d14046967e2ae76df1d0cbcfb3cbdfd57a515110e5c43bea0fc68be40129a2ca62a8c3c0cddee210b82b000006fe1b3d251a4634baf3 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
---
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
tidy: Skipping binary file check, read-only filesystem
Found 503 error codes
Highest error code: `E0791`
* 394 features
dependency exception `dunce` license has changed
    previously `CC0-1.0` now `CC0-1.0 OR MIT-0`
    update EXCEPTIONS for the new license
tidy error: invalid license `Apache-2.0` in `codespan-reporting 0.11.1 (registry+https://github.com/rust-lang/crates.io-index)`
tidy error: could not find allowed package `ansi_term`
Remove from PERMITTED_DEPENDENCIES list if it is no longer used.
tidy error: Dependency for rustc not explicitly permitted: windows_aarch64_gnullvm 0.42.1 (registry+https://github.com/rust-lang/crates.io-index)
tidy error: Dependency for rustc not explicitly permitted: windows_aarch64_msvc 0.42.1 (registry+https://github.com/rust-lang/crates.io-index)
tidy error: Dependency for rustc not explicitly permitted: windows_x86_64_gnu 0.42.1 (registry+https://github.com/rust-lang/crates.io-index)
tidy error: Dependency for rustc not explicitly permitted: windows_i686_gnu 0.42.1 (registry+https://github.com/rust-lang/crates.io-index)
tidy error: Dependency for rustc not explicitly permitted: windows-sys 0.42.0 (registry+https://github.com/rust-lang/crates.io-index)
tidy error: Dependency for rustc not explicitly permitted: overload 0.1.1 (registry+https://github.com/rust-lang/crates.io-index)
tidy error: Dependency for rustc not explicitly permitted: windows_x86_64_msvc 0.42.1 (registry+https://github.com/rust-lang/crates.io-index)
tidy error: Dependency for rustc not explicitly permitted: nu-ansi-term 0.46.0 (registry+https://github.com/rust-lang/crates.io-index)
tidy error: Dependency for rustc not explicitly permitted: windows_i686_msvc 0.42.1 (registry+https://github.com/rust-lang/crates.io-index)
tidy error: Dependency for rustc not explicitly permitted: windows_x86_64_gnullvm 0.42.1 (registry+https://github.com/rust-lang/crates.io-index)
some tidy checks failed

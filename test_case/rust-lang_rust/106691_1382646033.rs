plain
Successfully built 24fe501f8942
Successfully tagged rust-ci:latest
Built container sha256:24fe501f89429b43ce46a500bb1cd14d6dd74c6e42d99bf18b7f6b3c74269551
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
tidy error: dependency `urlqstring 0.3.5 (registry+https://github.com/rust-lang/crates.io-index)` does not define a license expression
tidy error: Dependencies for rustc not explicitly permitted:
* num 0.1.42 (registry+https://github.com/rust-lang/crates.io-index)
* num-iter 0.1.43 (registry+https://github.com/rust-lang/crates.io-index)
* urlqstring 0.3.5 (registry+https://github.com/rust-lang/crates.io-index)
some tidy checks failed

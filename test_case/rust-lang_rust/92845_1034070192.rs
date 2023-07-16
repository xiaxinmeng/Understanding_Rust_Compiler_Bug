plain
Successfully built 2d51467bcfdc
Successfully tagged rust-ci:latest
Built container sha256:2d51467bcfdc65ab5fc867673278673cebf8a156143e1b252380b8831eaf281a
Uploading finished image to https://ci-caches.rust-lang.org/docker/9567406ca397026e6d08b99ab2979859301d063de191c2500807ab84c19e827d07128981858917062997c4fe33329efa040c86f471172809b1d85076deabb2ef
upload failed: - to s3://rust-lang-ci-sccache2/docker/9567406ca397026e6d08b99ab2979859301d063de191c2500807ab84c19e827d07128981858917062997c4fe33329efa040c86f471172809b1d85076deabb2ef Unable to locate credentials
[CI_JOB_NAME=mingw-check]
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
    Finished release [optimized] target(s) in 7.67s
tidy check
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
tidy error: /checkout/library/std/src/personality.rs:17: platform-specific cfg: cfg(target_os = "emscripten")
tidy error: /checkout/library/std/src/personality.rs:19: platform-specific cfg: cfg(target_env = "msvc")
tidy error: /checkout/library/std/src/personality.rs:28: platform-specific cfg: cfg(any(
        all(target_family = "windows", target_env = "gnu"),
        target_os = "psp",
        target_os = "solid_asp3",
        all(target_family = "unix", not(target_os = "espidf")),
        all(target_vendor = "fortanix", target_env = "sgx"),
    ))
tidy error: /checkout/library/std/src/personality/gcc.rs:81: platform-specific cfg: cfg(all(target_arch = "arm", not(target_os = "ios"), not(target_os = "netbsd")))
tidy error: /checkout/library/std/src/personality/gcc.rs:219: platform-specific cfg: cfg(all(windows, target_arch = "x86_64", target_env = "gnu"))
tidy error: /checkout/library/std/src/personality/dwarf/eh.rs:52: platform-specific cfg: cfg!(all(target_os = "ios", target_arch = "arm"))
* highest error code: E0787
Found 503 error codes
Found 0 error codes with no tests
Done!

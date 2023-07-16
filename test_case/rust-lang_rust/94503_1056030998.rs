plain
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
    Finished release [optimized] target(s) in 9.43s
tidy check
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
tidy error: /checkout/library/core/src/ffi/mod.rs:64: platform-specific cfg: cfg(any(target_pointer_width = "32", windows))
tidy error: /checkout/library/core/src/ffi/mod.rs:67: platform-specific cfg: cfg(any(target_pointer_width = "32", windows))
tidy error: /checkout/library/core/src/ffi/mod.rs:70: platform-specific cfg: cfg(all(target_pointer_width = "64", not(windows)))
tidy error: /checkout/library/core/src/ffi/mod.rs:73: platform-specific cfg: cfg(all(target_pointer_width = "64", not(windows)))
tidy error: /checkout/library/core/src/ffi/mod.rs:103: platform-specific cfg: cfg(any(
            all(
                target_os = "linux",
                any(
                    target_arch = "aarch64",
                    target_arch = "arm",
                    target_arch = "hexagon",
                    target_arch = "powerpc",
                    target_arch = "powerpc64",
                    target_arch = "s390x",
                    target_arch = "riscv64",
                    target_arch = "riscv32"
            ),
            ),
            all(target_os = "android", any(target_arch = "aarch64", target_arch = "arm")),
            all(target_os = "l4re", target_arch = "x86_64"),
            all(
                target_os = "freebsd",
                any(
                    target_arch = "aarch64",
                    target_arch = "arm",
                    target_arch = "powerpc",
                    target_arch = "powerpc64",
                    target_arch = "riscv64"
            ),
            all(
            all(
                target_os = "netbsd",
                any(target_arch = "aarch64", target_arch = "arm", target_arch = "powerpc")
            ),
            all(target_os = "openbsd", target_arch = "aarch64"),
            all(
                target_os = "vxworks",
                any(
                    target_arch = "aarch64",
                    target_arch = "arm",
                    target_arch = "powerpc64",
                    target_arch = "powerpc"
            ),
            ),
            all(target_os = "fuchsia", target_arch = "aarch64")
        ))
tidy error: /checkout/library/core/src/ffi/mod.rs:192: platform-specific cfg: cfg(any(
    all(not(target_arch = "aarch64"), not(target_arch = "powerpc"), not(target_arch = "x86_64")),
    all(target_arch = "aarch64", any(target_os = "macos", target_os = "ios")),
    target_family = "wasm",
    target_arch = "asmjs",
))
))
tidy error: /checkout/library/core/src/ffi/mod.rs:215: platform-specific cfg: cfg(any(
    all(not(target_arch = "aarch64"), not(target_arch = "powerpc"), not(target_arch = "x86_64")),
    all(target_arch = "aarch64", any(target_os = "macos", target_os = "ios")),
    target_family = "wasm",
    target_arch = "asmjs",
))
))
tidy error: /checkout/library/core/src/ffi/mod.rs:239: platform-specific cfg: cfg(all(
    target_arch = "aarch64",
    not(any(target_os = "macos", target_os = "ios")),
    not(windows)
))
tidy error: /checkout/library/core/src/ffi/mod.rs:263: platform-specific cfg: cfg(all(target_arch = "powerpc", not(windows)))
tidy error: /checkout/library/core/src/ffi/mod.rs:283: platform-specific cfg: cfg(all(target_arch = "x86_64", not(windows)))
tidy error: /checkout/library/core/src/ffi/mod.rs:311: platform-specific cfg: cfg(any(
        all(
            not(target_arch = "aarch64"),
            not(target_arch = "powerpc"),
            not(target_arch = "x86_64")
        ),
        all(target_arch = "aarch64", any(target_os = "macos", target_os = "ios")),
        target_family = "wasm",
        target_arch = "asmjs",
    ))
    ))
tidy error: /checkout/library/core/src/ffi/mod.rs:324: platform-specific cfg: cfg(all(
        any(target_arch = "aarch64", target_arch = "powerpc", target_arch = "x86_64"),
        any(not(target_arch = "aarch64"), not(any(target_os = "macos", target_os = "ios"))),
        not(target_family = "wasm"),
        not(target_arch = "asmjs"),
        not(windows)
    ))
tidy error: /checkout/library/core/src/ffi/mod.rs:336: platform-specific cfg: cfg(any(
    all(not(target_arch = "aarch64"), not(target_arch = "powerpc"), not(target_arch = "x86_64")),
    all(target_arch = "aarch64", any(target_os = "macos", target_os = "ios")),
    target_family = "wasm",
    target_arch = "asmjs",
))
))
tidy error: /checkout/library/core/src/ffi/mod.rs:357: platform-specific cfg: cfg(all(
    any(target_arch = "aarch64", target_arch = "powerpc", target_arch = "x86_64"),
    any(not(target_arch = "aarch64"), not(any(target_os = "macos", target_os = "ios"))),
    not(target_family = "wasm"),
    not(target_arch = "asmjs"),
    not(windows)
* 629 error codes
* highest error code: E0787
Found 504 error codes
Found 0 error(s) in error codes

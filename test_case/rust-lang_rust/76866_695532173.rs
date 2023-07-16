
$ rg max_atomic_width | rg 128
compiler/rustc_target/src/spec/aarch64_linux_android.rs:    base.max_atomic_width = Some(128);
compiler/rustc_target/src/spec/aarch64_unknown_none.rs:        max_atomic_width: Some(128),
compiler/rustc_target/src/spec/aarch64_unknown_linux_musl.rs:    base.max_atomic_width = Some(128);
compiler/rustc_target/src/spec/aarch64_apple_ios.rs:            max_atomic_width: Some(128),
compiler/rustc_target/src/spec/aarch64_unknown_netbsd.rs:    base.max_atomic_width = Some(128);
compiler/rustc_target/src/spec/aarch64_apple_darwin.rs:    base.max_atomic_width = Some(128);
compiler/rustc_target/src/spec/x86_64_apple_darwin.rs:    base.max_atomic_width = Some(128); // core2 support cmpxchg16b
compiler/rustc_target/src/spec/aarch64_unknown_none_softfloat.rs:        max_atomic_width: Some(128),
compiler/rustc_target/src/spec/aarch64_unknown_freebsd.rs:    base.max_atomic_width = Some(128);
compiler/rustc_target/src/spec/aarch64_unknown_linux_gnu.rs:    base.max_atomic_width = Some(128);
compiler/rustc_target/src/spec/aarch64_apple_tvos.rs:            max_atomic_width: Some(128),
compiler/rustc_target/src/spec/aarch64_unknown_hermit.rs:    base.max_atomic_width = Some(128);
compiler/rustc_target/src/spec/aarch64_unknown_cloudabi.rs:    base.max_atomic_width = Some(128);
compiler/rustc_target/src/spec/aarch64_wrs_vxworks.rs:    base.max_atomic_width = Some(128);
compiler/rustc_target/src/spec/aarch64_unknown_openbsd.rs:    base.max_atomic_width = Some(128);
compiler/rustc_target/src/spec/aarch64_unknown_redox.rs:    base.max_atomic_width = Some(128);
compiler/rustc_target/src/spec/aarch64_fuchsia.rs:    base.max_atomic_width = Some(128);

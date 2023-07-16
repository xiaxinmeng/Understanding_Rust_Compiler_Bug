plain
   Compiling serde_json v1.0.57
   Compiling same-file v1.0.6
   Compiling fnv v1.0.7
   Compiling unicode-width v0.1.8
   Compiling cc v1.0.59 (https://github.com/alexcrichton/cc-rs?rev=192a84d9313210e09f2af1d7a8c27f70bd6a0f6e#192a84d9)
   Compiling opener v0.4.1
   Compiling build_helper v0.1.0 (/checkout/src/build_helper)
   Compiling thread_local v1.0.1
   Compiling getopts v0.2.21
---
   Compiling ignore v0.4.16
   Compiling toml v0.5.6
   Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
    Finished dev [unoptimized + debuginfo] target(s) in 48.50s
[src/bootstrap/config.rs:593] &config.cmd = Build {
    paths: [
        "nonexistent/path/to/trigger/cargo/metadata",
    ],
Build completed successfully in 0:00:58
bin_root (bootstrap.RustBuild)
Doctest: bootstrap.RustBuild.bin_root ... ok
bootstrap_binary (bootstrap.RustBuild)
---
DirectMap4k:      180160 kB
DirectMap2M:     6111232 kB
DirectMap1G:    54525952 kB
    Finished dev [unoptimized + debuginfo] target(s) in 0.18s
[src/bootstrap/config.rs:593] &config.cmd = Dist {
    paths: [],
}
[src/bootstrap/config.rs:598] flags.stage = None
[src/bootstrap/config.rs:598] dbg!(flags . stage).or(build.dist_stage).unwrap_or(2) = 2
Building stage0 tool unstable-book-gen (x86_64-unknown-linux-gnu)
---
   Compiling proc-macro2 v1.0.19
   Compiling unicode-xid v0.2.1
   Compiling scopeguard v1.1.0
   Compiling syn v1.0.38
   Compiling cc v1.0.59 (https://github.com/alexcrichton/cc-rs?rev=192a84d9313210e09f2af1d7a8c27f70bd6a0f6e#192a84d9)
   Compiling maybe-uninit v2.0.0
   Compiling semver-parser v0.7.0
   Compiling smallvec v1.4.2
   Compiling log v0.4.11
---
[100%] Built target clang_rt.tsan-x86_64
cargo:root=/checkout/obj/build/x86_64-unknown-linux-gnu/native/sanitizers
 finished in 13.232
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling cc v1.0.59 (https://github.com/alexcrichton/cc-rs?rev=192a84d9313210e09f2af1d7a8c27f70bd6a0f6e#192a84d9)
   Compiling libc v0.2.77
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.35
   Compiling unwind v0.0.0 (/checkout/library/unwind)
---
   Compiling scopeguard v1.1.0
   Compiling unicode-xid v0.2.1
   Compiling syn v1.0.38
   Compiling byteorder v1.3.4
   Compiling cc v1.0.59 (https://github.com/alexcrichton/cc-rs?rev=192a84d9313210e09f2af1d7a8c27f70bd6a0f6e#192a84d9)
   Compiling log v0.4.11
   Compiling smallvec v1.4.2
   Compiling semver-parser v0.7.0
   Compiling instant v0.1.6
---
Uplifting stage1 std (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Copying stage2 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
 Documenting core v0.0.0 (/checkout/library/core)
    Finished release [optimized + debuginfo] target(s) in 10.69s
   Compiling cc v1.0.59 (https://github.com/alexcrichton/cc-rs?rev=192a84d9313210e09f2af1d7a8c27f70bd6a0f6e#192a84d9)
   Compiling compiler_builtins v0.1.35
    Checking rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
 Documenting alloc v0.0.0 (/checkout/library/alloc)
    Finished release [optimized + debuginfo] target(s) in 18.62s
---
   Compiling unicode-xid v0.2.1
    Checking scopeguard v1.1.0
    Checking smallvec v1.4.2
   Compiling byteorder v1.3.4
   Compiling cc v1.0.59 (https://github.com/alexcrichton/cc-rs?rev=192a84d9313210e09f2af1d7a8c27f70bd6a0f6e#192a84d9)
   Compiling semver-parser v0.7.0
   Compiling log v0.4.11
    Checking hashbrown v0.9.0
   Compiling getrandom v0.1.14
---
   Compiling maybe-uninit v2.0.0
   Compiling pkg-config v0.3.18
   Compiling scopeguard v1.1.0
   Compiling rayon-core v1.7.1
   Compiling cc v1.0.59 (https://github.com/alexcrichton/cc-rs?rev=192a84d9313210e09f2af1d7a8c27f70bd6a0f6e#192a84d9)
   Compiling adler v0.2.3
   Compiling yaml-rust v0.3.5
   Compiling either v1.6.0
   Compiling crossbeam-utils v0.7.2

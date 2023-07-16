plain
   Compiling fnv v1.0.7
   Compiling serde_json v1.0.57
   Compiling itoa v0.4.6
   Compiling unicode-width v0.1.8
   Compiling cc v1.0.59 (https://github.com/alexcrichton/cc-rs?rev=192a84d9313210e09f2af1d7a8c27f70bd6a0f6e#192a84d9)
   Compiling build_helper v0.1.0 (/checkout/src/build_helper)
   Compiling thread_local v1.0.1
   Compiling walkdir v2.3.1
   Compiling getopts v0.2.21
---
   Compiling ignore v0.4.16
   Compiling toml v0.5.6
   Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
    Finished dev [unoptimized + debuginfo] target(s) in 50.37s
[src/bootstrap/config.rs:593] &config.cmd = Build {
    paths: [
        "nonexistent/path/to/trigger/cargo/metadata",
    ],
Build completed successfully in 0:01:02
bin_root (bootstrap.RustBuild)
Doctest: bootstrap.RustBuild.bin_root ... ok
bootstrap_binary (bootstrap.RustBuild)
---
DirectMap4k:      223168 kB
DirectMap2M:     7116800 kB
DirectMap1G:    52428800 kB
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
[src/bootstrap/config.rs:593] &config.cmd = Dist {
    paths: [],
}
[src/bootstrap/config.rs:598] flags.stage = None
[src/bootstrap/config.rs:598] dbg!(flags . stage).or(build.dist_stage).unwrap_or(2) = 2
Building stage0 tool unstable-book-gen (x86_64-unknown-linux-gnu)
---
   Compiling lazy_static v1.4.0
   Compiling proc-macro2 v1.0.19
   Compiling unicode-xid v0.2.1
   Compiling scopeguard v1.1.0
   Compiling cc v1.0.59 (https://github.com/alexcrichton/cc-rs?rev=192a84d9313210e09f2af1d7a8c27f70bd6a0f6e#192a84d9)
   Compiling syn v1.0.38
   Compiling maybe-uninit v2.0.0
   Compiling log v0.4.11
   Compiling semver-parser v0.7.0
---
[100%] Built target clang_rt.tsan-x86_64
cargo:root=/checkout/obj/build/x86_64-unknown-linux-gnu/native/sanitizers
 finished in 13.199
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling cc v1.0.59 (https://github.com/alexcrichton/cc-rs?rev=192a84d9313210e09f2af1d7a8c27f70bd6a0f6e#192a84d9)
   Compiling libc v0.2.77
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.35
   Compiling unwind v0.0.0 (/checkout/library/unwind)
---
   Compiling proc-macro2 v1.0.19
   Compiling scopeguard v1.1.0
   Compiling unicode-xid v0.2.1
   Compiling syn v1.0.38
   Compiling cc v1.0.59 (https://github.com/alexcrichton/cc-rs?rev=192a84d9313210e09f2af1d7a8c27f70bd6a0f6e#192a84d9)
   Compiling maybe-uninit v2.0.0
   Compiling log v0.4.11
   Compiling semver-parser v0.7.0
   Compiling smallvec v1.4.2
---
Uplifting stage1 std (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Copying stage2 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
 Documenting core v0.0.0 (/checkout/library/core)
    Finished release [optimized + debuginfo] target(s) in 10.95s
   Compiling cc v1.0.59 (https://github.com/alexcrichton/cc-rs?rev=192a84d9313210e09f2af1d7a8c27f70bd6a0f6e#192a84d9)
   Compiling compiler_builtins v0.1.35
    Checking rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
 Documenting alloc v0.0.0 (/checkout/library/alloc)
    Finished release [optimized + debuginfo] target(s) in 18.57s
---
   Compiling unicode-xid v0.2.1
    Checking scopeguard v1.1.0
    Checking smallvec v1.4.2
   Compiling byteorder v1.3.4
   Compiling cc v1.0.59 (https://github.com/alexcrichton/cc-rs?rev=192a84d9313210e09f2af1d7a8c27f70bd6a0f6e#192a84d9)
   Compiling log v0.4.11
   Compiling semver-parser v0.7.0
    Checking hashbrown v0.9.0
    Checking instant v0.1.6
---
  Downloaded crossbeam-queue v0.2.3
  Downloaded crc32fast v1.2.0
   Compiling maybe-uninit v2.0.0
   Compiling scopeguard v1.1.0
   Compiling cc v1.0.59 (https://github.com/alexcrichton/cc-rs?rev=192a84d9313210e09f2af1d7a8c27f70bd6a0f6e#192a84d9)
   Compiling pkg-config v0.3.18
   Compiling crc32fast v1.2.0
   Compiling adler v0.2.3
   Compiling either v1.6.0

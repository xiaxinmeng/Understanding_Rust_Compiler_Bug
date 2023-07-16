plain
    apt:
      update: true
travis_fold:start:git.checkout
travis_time:start:298e00d8
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/0f/7d/81e59502c95100bfb9010e6e04fe6dc8f03b4c11f5c63d79b9888ad4a412/awscli-1.15.20-py2.py3-none-any.whl (1.3MB)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/8d/bf/99c47b80476a890773d56233a890a4c30d0d5868e6c991dcc945f4735d75/botocore-1.10.20-py2.py3-none-any.whl (4.2MB)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/db/c8/7dcf9dbcb22429512708fe3a547f8b6101c0d02137acbd892505aee57adf/colorama-0.3.9-py2.py3-none-any.whl
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/e1/ae/baedc9cb175552e95f3395c43055a6a5e125ae4d48a1d7a924baca83e92e/rsa-3.4.2-py2.py3-none-any.whl (46kB)
Collecting s3transfer<0.2.0,>=0.1.12 (from awscli)
---
[00:07:04]    Compiling scopeguard v0.3.3
[00:07:04]    Compiling memoffset v0.2.1
[00:07:04]    Compiling libc v0.2.40
[00:07:04]    Compiling lazy_static v1.0.0
[00:07:04]    Compiling rustc-rayon-core v0.1.0 (https://github.com/Zoxc/rayon.git?branch=rustc#bf8d7a3d)
[00:07:05]    Compiling stable_deref_trait v1.0.0
[00:07:05]    Compiling bitflags v1.0.1
[00:07:05]    Compiling either v1.5.0
[00:07:05]    Compiling serialize v0.0.0 (file:///checkout/src/libserialize)
---
[00:07:42]    Compiling crossbeam-deque v0.2.0
[00:07:43]    Compiling rls-data v0.15.0
[00:07:50]    Compiling flate2 v1.0.1
[00:07:51]    Compiling backtrace v0.3.6
[00:07:58]    Compiling rustc-rayon v0.1.0 (https://github.com/Zoxc/rayon.git?branch=rustc#bf8d7a3d)
[00:08:04]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:08:04]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:08:09]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:09:54]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
---
[00:37:07]    Compiling libc v0.2.40
[00:37:07]    Compiling memoffset v0.2.1
[00:37:07]    Compiling lazy_static v1.0.0
[00:37:07]    Compiling scopeguard v0.3.3
[00:37:07]    Compiling rustc-rayon-core v0.1.0 (https://github.com/Zoxc/rayon.git?branch=rustc#bf8d7a3d)
[00:37:07]    Compiling smallvec v0.6.0
[00:37:07]    Compiling either v1.5.0
[00:37:08]    Compiling bitflags v1.0.1
[00:37:08]    Compiling serialize v0.0.0 (file:///checkout/src/libserialize)
---
[00:37:43]    Compiling rustc_apfloat v0.0.0 (file:///checkout/src/librustc_apfloat)
[00:37:47]    Compiling crossbeam-deque v0.2.0
[00:37:47]    Compiling parking_lot v0.5.5
[00:37:47]    Compiling flate2 v1.0.1
[00:37:56]    Compiling rustc-rayon v0.1.0 (https://github.com/Zoxc/rayon.git?branch=rustc#bf8d7a3d)
[00:38:04]    Compiling rustc_data_structures v0.0.0 (file:///checkout/src/librustc_data_structures)
[00:38:06]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:38:06]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:38:10]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
---
[01:04:36]    Compiling crossbeam-deque v0.2.0
[01:04:36]    Compiling rand v0.3.22
[01:04:36]    Compiling parking_lot_core v0.2.14
[01:04:36]    Compiling rustc_target v0.0.0 (file:///checkout/src/librustc_target)
[01:04:37]    Compiling rustc-rayon-core v0.1.0 (https://github.com/Zoxc/rayon.git?branch=rustc#bf8d7a3d)
[01:04:37]    Compiling parking_lot v0.5.5
[01:04:38]    Compiling rustc-rayon v0.1.0 (https://github.com/Zoxc/rayon.git?branch=rustc#bf8d7a3d)
[01:04:42]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[01:04:43]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[01:04:44]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[01:04:45]    Compiling syntax v0.0.0 (file:///checkout/src/libsyntax)
---
[01:05:14] warning: [i] cannot be resolved, ignoring it...
[01:05:14] 
[01:05:14] warning: [j] cannot be resolved, ignoring it...
[01:05:14] 
[01:05:14] thread '<unnamed>' panicked at 'assertion failed: gcx != 0', librustc/ty/context.rs:1904:9
[01:05:15] warning: [cfg] cannot be resolved, ignoring it...
[01:05:15] 
[01:05:15] warning: [rayon::prelude] cannot be resolved, ignoring it...
[01:05:15] 
---
[01:05:19] warning: [4] cannot be resolved, ignoring it...
[01:05:19] 
[01:05:19] warning: [::] cannot be resolved, ignoring it...
[01:05:19] 
[01:05:20] error[E0432]: unresolved import `self::job::handle_deadlock`
[01:05:20]   --> librustc/ty/maps/mod.rs:69:42
[01:05:20]    |
[01:05:20] 69 | pub use self::job::{QueryJob, QueryInfo, handle_deadlock};
[01:05:20]    |                                          ^^^^^^^^^^^^^^^ no `handle_deadlock` in `ty::maps::job`
[01:05:20] warning: [foo] cannot be resolved, ignoring it...
[01:05:20] 
[01:05:20] warning: [cfgspec] cannot be resolved, ignoring it...
[01:05:20] 
[01:05:20] 
[01:05:20] error[E0412]: cannot find type `QueryLatch` in this scope
[01:05:20]   --> librustc/ty/maps/job.rs:64:12
[01:05:20]    |
[01:05:20] 64 |     latch: QueryLatch,
[01:05:20] 
[01:05:23] error: Compilation failed, aborting rustdoc
[01:05:23] 
[01:05:23] error: Could not document `rustc`.
[01:05:23] error: Could not document `rustc`.
[01:05:23] 
[01:05:23] Caused by:
[01:05:23]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name rustc librustc/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/doc -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-5840050fc38f566e.rmeta --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-bd35db077c2a1fe7.rmeta --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-edfc458912c68b5e.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-f238f190be0480db.rmeta --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblazy_static-4d0e39d16eac666e.rmeta --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-728d0bc77ee95870.rmeta --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-0ab1cfa7599c8e72.rmeta --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-dc6071754b53bfc3.rmeta --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-dcbc090e090d72ff.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-b6219b91bd9f1a4b.rmeta --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-1e3de39a6952b3cd.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-a754233a5a04e2a5.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-601f1dc735baaf09.rmeta --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-e22db64f40fcd164.rmeta --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-29200166fc5b472a.rmeta --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-28d1803f8764f2a7.rmeta --extern rustc_rayon_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon_core-eb1c13b79d7d8078.rmeta --extern tempdir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libtempdir-4f63df90d33adf8d.rmeta --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-9245348800090e0a.rmeta --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-32f7d5933a5d8e5c.rmeta --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-a4f10411a32a3fb4.rmeta --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-cbd63665c01bbcea.rmeta --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-a67bc74a0c1c51cf.rmeta --document-private-items` (exit code: 101)
[01:06:26] error: build failed
[01:06:26] 
[01:06:26] 
[01:06:26] 
[01:06:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--no-deps" "-p" "rustc_incremental" "-p" "serialize" "-p" "rustc_resolve" "-p" "arena" "-p" "rustc_privacy" "-p" "rustc_target" "-p" "rustc_apfloat" "-p" "rustc_typeck" "-p" "rustc_traits" "-p" "rustc_driver" "-p" "syntax" "-p" "rustc_allocator" "-p" "fmt_macros" "-p" "rustc_plugin" "-p" "graphviz" "-p" "syntax_pos" "-p" "rustc_metadata" "-p" "rustc_platform_intrinsics" "-p" "rustc_errors" "-p" "proc_macro" "-p" "rustc_lint" "-p" "rustc" "-p" "rustc_save_analysis" "-p" "rustc_mir" "-p" "rustc_passes" "-p" "syntax_ext" "-p" "rustc_trans_utils" "-p" "rustc_borrowck" "-p" "rustc_data_structures"
[01:06:26] 
[01:06:26] 
[01:06:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[01:06:26] Build completed unsuccessfully in 1:00:42

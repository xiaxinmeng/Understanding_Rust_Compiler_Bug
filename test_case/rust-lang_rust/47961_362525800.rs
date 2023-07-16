
running: /data/semarie/build-rust/build_dir/build/bootstrap/debug/bootstrap dist --jobs=1
finding compilers
CC_x86_64-unknown-openbsd = "cc"
AR_x86_64-unknown-openbsd = "ar"
CXX_x86_64-unknown-openbsd = "c++"
running sanity check
learning about cargo
> Docs { stage: 2, host: "x86_64-unknown-openbsd" }
Dist docs (x86_64-unknown-openbsd)
  > UnstableBook { target: "x86_64-unknown-openbsd" }
    > UnstableBookGen { target: "x86_64-unknown-openbsd" }
      > Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-openbsd" } }
        > Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-openbsd" } }
          > Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-openbsd" } }
          < Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-openbsd" } }
          > Rustc { compiler: Compiler { stage: 0, host: "x86_64-unknown-openbsd" }, target: "x86_64-unknown-openbsd" }
            > Test { compiler: Compiler { stage: 0, host: "x86_64-unknown-openbsd" }, target: "x86_64-unknown-openbsd" }
              > Std { target: "x86_64-unknown-openbsd", compiler: Compiler { stage: 0, host: "x86_64-unknown-openbsd" } }
                > StartupObjects { compiler: Compiler { stage: 0, host: "x86_64-unknown-openbsd" }, target: "x86_64-unknown-openbsd" }
                < StartupObjects { compiler: Compiler { stage: 0, host: "x86_64-unknown-openbsd" }, target: "x86_64-unknown-openbsd" }
Building stage0 std artifacts (x86_64-unknown-openbsd -> x86_64-unknown-openbsd)
Dirty - /data/semarie/build-rust/build_dir/build/x86_64-unknown-openbsd/stage0-std
                > Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-openbsd" } }
                < Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-openbsd" } }
fatal: 'origin' does not appear to be a git repository
fatal: Impossible de lire le dépôt distant.

Veuillez vérifier que vous avez les droits d'accès
et que le dépôt existe.
thread 'main' panicked at 'command did not execute successfully: "git" "ls-remote" "origin" "beta"
expected success, got: exit code: 128', rustc-beta-src/src/build_helper/lib.rs:116:8
stack backtrace:
   0: <std::process::ExitStatus as core::fmt::Display>::fmt
   1: <std::process::ExitStatus as core::fmt::Display>::fmt
   2: <std::process::ExitStatus as core::fmt::Display>::fmt
   3: <std::process::ExitStatus as core::fmt::Display>::fmt
   4: <std::process::ExitStatus as core::fmt::Display>::fmt
   5: <std::process::ExitStatus as core::fmt::Display>::fmt
   6: <std::process::ExitStatus as core::fmt::Display>::fmt
   7: <serde::de::OneOf as core::fmt::Display>::fmt
   8: <bootstrap::cache::Interned<alloc::string::String> as core::fmt::Display>::fmt
   9: <bootstrap::cache::Interned<alloc::string::String> as core::fmt::Display>::fmt
  10: <bootstrap::compile::Assemble as core::fmt::Debug>::fmt
  11: <bootstrap::cache::Interned<alloc::string::String> as core::fmt::Display>::fmt
  12: __register_frame_info
  13: <bootstrap::compile::Std as bootstrap::builder::Step>::make_run
  14: __register_frame_info
  15: <bootstrap::compile::Test as bootstrap::builder::Step>::make_run
  16: __register_frame_info
  17: <bootstrap::compile::Rustc as bootstrap::builder::Step>::make_run
  18: __register_frame_info
  19: <bootstrap::compile::Rustc as bootstrap::builder::Step>::make_run
  20: __register_frame_info
  21: __register_frame_info
  22: <bootstrap::compile::Rustc as bootstrap::builder::Step>::make_run
  23: __register_frame_info
  24: __register_frame_info
  25: <bootstrap::doc::UnstableBookGen as bootstrap::builder::Step>::make_run
  26: __register_frame_info
  27: <bootstrap::doc::UnstableBook as bootstrap::builder::Step>::make_run
  28: __register_frame_info
  29: <bootstrap::doc::UnstableBook as bootstrap::builder::Step>::make_run
  30: __register_frame_info
  31: __register_frame_info
  32: __register_frame_info
  33: <bootstrap::dist::Docs as bootstrap::builder::Step>::make_run
  34: __register_frame_info
  35: <bootstrap::dist::Docs as bootstrap::builder::Step>::make_run
  36: __register_frame_info
  37: __register_frame_info
  38: __register_frame_info
  39: <bootstrap::cache::Interned<alloc::string::String> as core::fmt::Display>::fmt
  40: __register_frame_info
  41: <std::io::error::Error as core::fmt::Debug>::fmt
  42: <std::process::ExitStatus as core::fmt::Display>::fmt
  43: __register_frame_info
  44: 
Traceback (most recent call last):
  File "/data/semarie/build-rust/build_dir/rustc-beta-src/x.py", line 20, in <module>
    bootstrap.main()
  File "/data/semarie/build-rust/build_dir/rustc-beta-src/src/bootstrap/bootstrap.py", line 758, in main
    bootstrap()
  File "/data/semarie/build-rust/build_dir/rustc-beta-src/src/bootstrap/bootstrap.py", line 749, in bootstrap
    run(args, env=env, verbose=build.verbose)
  File "/data/semarie/build-rust/build_dir/rustc-beta-src/src/bootstrap/bootstrap.py", line 148, in run
    raise RuntimeError(err)
RuntimeError: failed to run: /data/semarie/build-rust/build_dir/build/bootstrap/debug/bootstrap dist --jobs=1

plain
travis_time:end:30c0963c:start=1543439071856231075,finish=1543439138331442859,duration=66475211784
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/4f/dd/d1b374af4e5f374173517e7d63e2f4489d3d2e9a45626df830c885412cc9/awscli-1.16.64-py2.py3-none-any.whl (1.4MB)
    0% |▎                               | 10kB 11.5MB/s eta 0:00:01
    1% |▌                               | 20kB 1.8MB/s eta 0:00:01
    2% |▊                               | 30kB 2.2MB/s eta 0:00:01
    2% |█                               | 40kB 2.0MB/s eta 0:00:01
---
[00:00:00] Attempting with retry: sh -c rm -f download-src-tools-lld.tar.gz &&         curl -sSL -o download-src-tools-lld.tar.gz https://github.com/rust-lang/lld/archive/1928c5eeb613a4c6d232fc47ae91914bbfd92a79.tar.gz
[00:00:00] rm 'src/tools/lldb'
[00:00:00] Attempting with retry: sh -c rm -f download-src-tools-lldb.tar.gz &&         curl -sSL -o download-src-tools-lldb.tar.gz https://github.com/rust-lang-nursery/lldb/archive/fdea743be550ed8d7b61b2c908944cdd1290a6ad.tar.gz
[00:00:00] rm 'src/tools/clang'
[00:00:00] Attempting with retry: sh -c git submodule deinit -f  src/tools/rust-installer src/liblibc src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/libcompiler_builtins src/tools/clippy src/tools/rustfmt src/tools/miri src/dlmalloc src/stdsimd src/libbacktrace src/doc/rustc-guide &&     git submodule sync &&     git submodule update -j 16 --init --recursive  src/tools/rust-installer src/liblibc src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/libcompiler_builtins src/tools/clippy src/tools/rustfmt src/tools/miri src/dlmalloc src/stdsimd src/libbacktrace src/doc/rustc-guide
[00:00:00] Cleared directory 'src/dlmalloc'
[00:00:00] Cleared directory 'src/doc/nomicon'
[00:00:00] Cleared directory 'src/doc/reference'
[00:00:00] Cleared directory 'src/doc/rustc-guide'
---
[00:00:00] Cleared directory 'src/tools/rustfmt'
[00:00:00] Submodule 'src/dlmalloc' (https://github.com/alexcrichton/dlmalloc-rs.git) registered for path 'src/dlmalloc'
[00:00:00] Submodule 'src/doc/nomicon' (https://github.com/rust-lang-nursery/nomicon.git) registered for path 'src/doc/nomicon'
[00:00:00] Submodule 'src/doc/reference' (https://github.com/rust-lang-nursery/reference.git) registered for path 'src/doc/reference'
[00:00:00] Submodule 'src/doc/rustc-guide' (https://github.com/rust-lang/rustc-guide.git) registered for path 'src/doc/rustc-guide'
[00:00:00] Submodule 'src/libcompiler_builtins' (https://github.com/rust-lang-nursery/compiler-builtins.git) registered for path 'src/libcompiler_builtins'
[00:00:00] Submodule 'src/liblibc' (https://github.com/rust-lang/libc.git) registered for path 'src/liblibc'
[00:00:00] Submodule 'src/stdsimd' (https://github.com/rust-lang-nursery/stdsimd.git) registered for path 'src/stdsimd'
[00:00:00] Submodule 'src/tools/cargo' (https://github.com/rust-lang/cargo.git) registered for path 'src/tools/cargo'
---
[00:00:01] Cloning into '/home/travis/build/rust-lang/rust/src/tools/rust-installer'...
[00:00:01] Cloning into '/home/travis/build/rust-lang/rust/src/doc/reference'...
[00:00:02] Cloning into '/home/travis/build/rust-lang/rust/src/libcompiler_builtins'...
[00:00:02] Cloning into '/home/travis/build/rust-lang/rust/src/libbacktrace'...
[00:00:02] Cloning into '/home/travis/build/rust-lang/rust/src/doc/rustc-guide'...
[00:00:05] Cloning into '/home/travis/build/rust-lang/rust/src/tools/miri'...
[00:00:05] Cloning into '/home/travis/build/rust-lang/rust/src/stdsimd'...
[00:00:05] Cloning into '/home/travis/build/rust-lang/rust/src/tools/rustfmt'...
[00:00:05] Cloning into '/home/travis/build/rust-lang/rust/src/tools/rls'...
[00:00:05] Cloning into '/home/travis/build/rust-lang/rust/src/tools/rls'...
[00:00:05] Cloning into '/home/travis/build/rust-lang/rust/src/tools/clippy'...
[00:00:06] Cloning into '/home/travis/build/rust-lang/rust/src/tools/cargo'...
[00:00:06] Submodule path 'src/dlmalloc': checked out 'c99638dc2ecfc750cc1656f6edb2bd062c1e0981'
[00:00:06] Submodule path 'src/doc/nomicon': checked out 'f8a4e96feb2e5a6ed1ef170ad40e3509a7755cb4'
[00:00:06] Submodule path 'src/doc/reference': checked out '60077efda319c95a89fe39609803c5433567adbf'
[00:00:06] Submodule path 'src/doc/rustc-guide': checked out '3a804956e3c28d7e44e38804207a00013594e1d3'
[00:00:06] Submodule path 'src/libcompiler_builtins': checked out 'fe74674f6e4be76d47b66f67d529ebf4186f4eb1'
[00:00:06] Submodule 'compiler-rt' (https://github.com/rust-lang/compiler-rt) registered for path 'src/libcompiler_builtins/compiler-rt'
[00:00:06] Submodule 'libm' (https://github.com/rust-lang-nursery/libm) registered for path 'src/libcompiler_builtins/libm'
[00:00:06] Cloning into '/home/travis/build/rust-lang/rust/src/libcompiler_builtins/compiler-rt'...
---
[00:06:25]    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:06:34]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:11:52]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:11:52]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:12:02] error[E0063]: missing field `two_phase` in initializer of `rustc::mir::StatementKind<'_>`
[00:12:02]     |
[00:12:02]     |
[00:12:02] 232 |                 kind: StatementKind::Retag { fn_entry: true, place: dropee_ptr.clone() },
[00:12:02]     |                       ^^^^^^^^^^^^^^^^^^^^ missing `two_phase`
[00:12:07] error: aborting due to previous error
[00:12:07] 
[00:12:07] For more information about this error, try `rustc --explain E0063`.
[00:12:07] error: Could not compile `rustc_mir`.
[00:12:07] error: Could not compile `rustc_mir`.
[00:12:07] warning: build failed, waiting for other jobs to finish...
[00:13:32] error: build failed
[00:13:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:13:32] expected success, got: exit code: 101
[00:13:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:13:32] Build completed unsuccessfully in 0:10:13
[00:13:32] make: *** [all] Error 1
[00:13:32] Makefile:28: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:089e484a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Nov 28 21:19:19 UTC 2018

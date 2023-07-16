plain
travis_time:end:10b2185c:start=1543441061320444810,finish=1543441115306559732,duration=53986114922
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
    0% |▎                               | 10kB 14.4MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
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
[00:00:01] Cloning into '/home/travis/build/rust-lang/rust/src/libcompiler_builtins'...
[00:00:01] Cloning into '/home/travis/build/rust-lang/rust/src/doc/reference'...
[00:00:02] Cloning into '/home/travis/build/rust-lang/rust/src/libbacktrace'...
[00:00:02] Cloning into '/home/travis/build/rust-lang/rust/src/doc/rustc-guide'...
[00:00:02] Cloning into '/home/travis/build/rust-lang/rust/src/tools/miri'...
[00:00:02] Cloning into '/home/travis/build/rust-lang/rust/src/liblibc'...
[00:00:04] Cloning into '/home/travis/build/rust-lang/rust/src/stdsimd'...
[00:00:04] Cloning into '/home/travis/build/rust-lang/rust/src/tools/rustfmt'...
[00:00:04] Cloning into '/home/travis/build/rust-lang/rust/src/tools/rustfmt'...
[00:00:04] Cloning into '/home/travis/build/rust-lang/rust/src/tools/clippy'...
[00:00:04] Cloning into '/home/travis/build/rust-lang/rust/src/tools/cargo'...
[00:00:05] Submodule path 'src/dlmalloc': checked out 'c99638dc2ecfc750cc1656f6edb2bd062c1e0981'
[00:00:05] Submodule path 'src/doc/nomicon': checked out 'f8a4e96feb2e5a6ed1ef170ad40e3509a7755cb4'
[00:00:05] Submodule path 'src/doc/reference': checked out '60077efda319c95a89fe39609803c5433567adbf'
[00:00:05] Submodule path 'src/doc/rustc-guide': checked out '3a804956e3c28d7e44e38804207a00013594e1d3'
[00:00:05] Submodule path 'src/libcompiler_builtins': checked out 'fe74674f6e4be76d47b66f67d529ebf4186f4eb1'
[00:00:05] Submodule 'compiler-rt' (https://github.com/rust-lang/compiler-rt) registered for path 'src/libcompiler_builtins/compiler-rt'
[00:00:05] Submodule 'libm' (https://github.com/rust-lang-nursery/libm) registered for path 'src/libcompiler_builtins/libm'
[00:00:05] Cloning into '/home/travis/build/rust-lang/rust/src/libcompiler_builtins/compiler-rt'...
---
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:44:09] 
[00:44:09] running 5065 tests
[00:44:11] ..............................F..................................................................... 100/5065
[00:44:17] .................................................................................................... 300/5065
[00:44:19] .................................................................................................... 400/5065
[00:44:22] .................................................................................................... 500/5065
[00:44:26] ..............................i..................................................................... 600/5065
---
[00:46:35] .................................................................................................... 4800/5065
[00:46:37] .................................................................................................... 4900/5065
[00:46:40] .................................................................................................... 5000/5065
[00:46:42] The actual stderr differed from the expected stderr.
[00:46:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/always-inhabited-union-ref/always-inhabited-union-ref.stderr
[00:46:42] To update references, rerun the tests and pass the `--bless` flag
[00:46:42] To only update this specific test, also pass `--test-args always-inhabited-union-ref.rs`
[00:46:42] error: 1 errors occurred comparing output.
[00:46:42] status: exit code: 1
[00:46:42] status: exit code: 1
[00:46:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/always-inhabited-union-ref.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/always-inhabited-union-ref/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/always-inhabited-union-ref/auxiliary" "-A" "unused"
[00:46:42] ------------------------------------------
[00:46:42] 
[00:46:42] ------------------------------------------
[00:46:42] stderr:
[00:46:42] stderr:
[00:46:42] ------------------------------------------
[00:46:42] {"message":"non-exhaustive patterns: type `&'static !` is non-empty","code":{"code":"E0004","explanation":"\nThis error indicates that the compiler cannot guarantee a matching pattern for\none or more possible inputs to a match expression. Guaranteed matches are\nrequired in order to assign values to match expressions, or alternatively,\ndetermine the flow of execution. Erroneous code example:\n\n
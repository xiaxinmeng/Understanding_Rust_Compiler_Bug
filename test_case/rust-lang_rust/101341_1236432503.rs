plain
 ---> 87e740682e3e
Step 13/14 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
 ---> Using cache
 ---> 95d616262d33
Step 14/14 : ENV SCRIPT python3 ../x.py --stage 2 test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test --stage 2 src/tools/tidy &&            python3 ../x.py test --stage 0 core alloc std test proc_macro &&            RUSTDOCFLAGS="--document-private-items" python3 ../x.py doc --stage 0 library/test &&            /scripts/validate-toolstate.sh &&            /scripts/validate-error-codes.sh &&            reuse lint &&            es-check es6 ../src/librustdoc/html/static/js/*.js &&            eslint -c ../src/librustdoc/html/static/.eslintrc.js ../src/librustdoc/html/static/js/*.js
 ---> dbb48a19ed7b
Successfully built dbb48a19ed7b
Successfully tagged rust-ci:latest
Built container sha256:dbb48a19ed7b4eba9d8604f87abe7dd2740c8f84b5a23780352c98ed47afecfb
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
..............................
failures:

---- string::test_try_reserve stdout ----
thread 'string::test_try_reserve' panicked at 'isize::MAX shouldn't trigger an overflow!', library/alloc/tests/string.rs:702:13
---- vec::test_try_reserve stdout ----
---- vec::test_try_reserve stdout ----
thread 'vec::test_try_reserve' panicked at 'isize::MAX shouldn't trigger an overflow!', library/alloc/tests/vec.rs:1646:13

failures:
    string::test_try_reserve
    vec::test_try_reserve

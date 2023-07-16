plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8f4b7f84864484a7bf31766abe9204da3cbe65b3)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3040325909b538d8ad81ad89a04b7a91b109c313)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 14753 tests
..........................................ii............................................    88/14753
.................................................................................iiiiiii   176/14753
iiiiiiii...F.................i..................i.......................................   264/14753
........................................................................................   440/14753
........................................................................................   528/14753
........................................................................................   616/14753
........................................................................................   704/14753
---
---- [ui] tests/ui/abi/stack-probes.rs stdout ----

error: test run failed!
status: exit status: 101
command: cd "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/stack-probes" && RUST_TEST_THREADS="16" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/stack-probes/a"
--- stdout -------------------------------
status: signal: 6 (SIGABRT) (core dumped)
stderr: 
thread '<unknown>' has overflowed its stack
fatal runtime error: stack overflow


status: signal: 11 (SIGSEGV) (core dumped)
stderr:
------------------------------------------
--- stderr -------------------------------
--- stderr -------------------------------
thread 'main' panicked at 'assertion failed: stderr.contains(\"has overflowed its stack\\n\")', fake-test-src-base/abi/stack-probes.rs:106:5
------------------------------------------


---- [ui] tests/ui/abi/stack-probes-lto.rs stdout ----
---- [ui] tests/ui/abi/stack-probes-lto.rs stdout ----

error: test run failed!
status: exit status: 101
command: cd "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/stack-probes-lto" && RUST_TEST_THREADS="16" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/stack-probes-lto/a"
--- stdout -------------------------------
status: signal: 6 (SIGABRT) (core dumped)
stderr: 
thread '<unknown>' has overflowed its stack
fatal runtime error: stack overflow


status: signal: 11 (SIGSEGV) (core dumped)
stderr:
------------------------------------------
--- stderr -------------------------------
--- stderr -------------------------------
thread 'main' panicked at 'assertion failed: stderr.contains(\"has overflowed its stack\\n\")', fake-test-src-base/abi/stack-probes.rs:106:5
------------------------------------------




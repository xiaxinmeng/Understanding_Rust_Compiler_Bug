plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:0fdabd83e1d3faaa8e9cfd7c00031e3a92997344)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---

---- [ui] tests/ui/hygiene/panic-location.rs stdout ----
diff of run.stderr:

- thread 'main' panicked at 'capacity overflow', library/alloc/src/raw_vec.rs:525:5
+ thread 'main' panicked at 'capacity overflow', library/alloc/src/raw_vec.rs:528:5
3 


The actual run.stderr differed from the expected run.stderr.
The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/panic-location/panic-location.run.stderr
error: 1 errors occurred comparing run output.
status: exit status: 101
status: exit status: 101
command: cd "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/panic-location" && RUST_BACKTRACE="0" RUST_TEST_THREADS="16" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/panic-location/a"
stdout: none
--- stderr -------------------------------
thread 'main' panicked at 'capacity overflow', library/alloc/src/raw_vec.rs:528:5
------------------------------------------




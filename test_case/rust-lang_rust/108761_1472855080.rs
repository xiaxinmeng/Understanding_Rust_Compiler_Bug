plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3fb2b44a4eaebb9ed8086446bde46c27199ef5ed)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---

---- [ui] tests/ui/hygiene/panic-location.rs stdout ----
diff of run.stderr:

- thread 'main' panicked at 'capacity overflow', library/alloc/src/raw_vec.rs:600:5
+ thread 'main' panicked at 'capacity overflow', library/alloc/src/raw_vec.rs:601:5
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
thread 'main' panicked at 'capacity overflow', library/alloc/src/raw_vec.rs:601:5
------------------------------------------




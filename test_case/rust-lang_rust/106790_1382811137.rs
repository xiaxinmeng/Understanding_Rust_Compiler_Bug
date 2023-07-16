plain

---- [ui] checkout/tests/ui/hygiene/panic-location.rs stdout ----
diff of run.stderr:

- thread 'main' panicked at 'capacity overflow', library/alloc/src/raw_vec.rs:535:5
+ thread 'main' panicked at 'capacity overflow', library/alloc/src/raw_vec.rs:529:5
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
thread 'main' panicked at 'capacity overflow', library/alloc/src/raw_vec.rs:529:5
------------------------------------------




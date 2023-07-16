
Testing libtest stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling term v0.0.0 (file:///checkout/src/libterm)
   Compiling test v0.0.0 (file:///checkout/src/libtest)
error[E0308]: mismatched types
   --> /checkout/src/libtest/stats.rs:887:5
    |
887 | /     pub fn sum_three_items(b: &mut Bencher) {
888 | |         b.iter(|| {
889 | |             [1e20f64, 1.5f64, -1e20f64].sum();
890 | |         })
891 | |     }
    | |_____^ expected struct `__test::test::Bencher`, found struct `Bencher`
    |
    = note: expected type `fn(&mut __test::test::Bencher)`
               found type `fn(&mut Bencher) {stats::bench::sum_three_items}`

error[E0308]: mismatched types
   --> /checkout/src/libtest/stats.rs:893:5
    |
893 | /     pub fn sum_many_f64(b: &mut Bencher) {
894 | |         let nums = [-1e30f64, 1e60, 1e30, 1.0, -1e60];
895 | |         let v = (0..500).map(|i| nums[i % 5]).collect::<Vec<_>>();
896 | |
...   |
899 | |         })
900 | |     }
    | |_____^ expected struct `__test::test::Bencher`, found struct `Bencher`
    |
    = note: expected type `fn(&mut __test::test::Bencher)`
               found type `fn(&mut Bencher) {stats::bench::sum_many_f64}`

error[E0308]: mismatched types
   --> /checkout/src/libtest/stats.rs:903:5
    |
903 |     pub fn no_iter(_: &mut Bencher) {}
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `__test::test::Bencher`, found struct `Bencher`
    |
    = note: expected type `fn(&mut __test::test::Bencher)`
               found type `fn(&mut Bencher) {stats::bench::no_iter}`

error: aborting due to 3 previous errors

error: Could not compile `test`.
warning: build failed, waiting for other jobs to finish...
error: build failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "-j" "4" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "-p" "test:0.0.0" "-p" "term:0.0.0" "--" "--quiet"
expected success, got: exit code: 101


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
Build completed unsuccessfully in 0:23:20
Makefile:54: recipe for target 'check' failed
make: *** [check] Error 1



% for d in sep24 oct01 oct05 oct10 ; do ls -Fd rust-$d-* ; ./rust-$d-*/objdir-opt/x86_64-apple-darwin/stage2/bin/rustc -Aunused-imports --opt-level=3 --test /tmp/b.rs && /tmp/b --bench ; done
rust-sep24-a7d68adbdd75300a4a10b2dcc59e1048f613db48/
warning: no debug symbols in executable (-arch x86_64)

running 7 tests
test test::push ... ignored
test bench::empty ... bench: 5 ns/iter (+/- 0)
test bench::empty_std ... bench: 58 ns/iter (+/- 7)
test bench::push ... bench: 37438 ns/iter (+/- 3493)
test bench::push_preallocated ... bench: 34575 ns/iter (+/- 10314)
test bench::push_preallocated_std ... bench: 34751 ns/iter (+/- 5529)
test bench::push_std ... bench: 43656 ns/iter (+/- 4786)

test result: ok. 0 passed; 0 failed; 1 ignored; 6 measured

rust-oct01-88b0b511beed1599c5bddaf05b9cd0f98bd714ca/
warning: no debug symbols in executable (-arch x86_64)

running 7 tests
test test::push ... ignored
test bench::empty ... bench: 5 ns/iter (+/- 0)
test bench::empty_std ... bench: 59 ns/iter (+/- 6)
test bench::push ... bench: 11623 ns/iter (+/- 2518)
test bench::push_preallocated ... bench: 5882 ns/iter (+/- 744)
test bench::push_preallocated_std ... bench: 37822 ns/iter (+/- 5728)
test bench::push_std ... bench: 42381 ns/iter (+/- 20040)

test result: ok. 0 passed; 0 failed; 1 ignored; 6 measured

rust-oct05-549f70989e3423642bacc41bb6c58ed81d591c03/
warning: no debug symbols in executable (-arch x86_64)

running 7 tests
test test::push ... ignored
test bench::empty ... bench: 5 ns/iter (+/- 0)
test bench::empty_std ... bench: 58 ns/iter (+/- 9)
test bench::push ... bench: 11789 ns/iter (+/- 3440)
test bench::push_preallocated ... bench: 5839 ns/iter (+/- 2173)
test bench::push_preallocated_std ... bench: 37940 ns/iter (+/- 3766)
test bench::push_std ... bench: 42484 ns/iter (+/- 5002)

test result: ok. 0 passed; 0 failed; 1 ignored; 6 measured

rust-oct10-2e1df8e35bb0185dc35f5e7976c0b46608c32404/
warning: no debug symbols in executable (-arch x86_64)

running 7 tests
test test::push ... ignored
test bench::empty ... bench: 5 ns/iter (+/- 0)
test bench::empty_std ... bench: 59 ns/iter (+/- 14)
test bench::push ... bench: 11881 ns/iter (+/- 2696)
test bench::push_preallocated ... bench: 5827 ns/iter (+/- 457)
test bench::push_preallocated_std ... bench: 37639 ns/iter (+/- 4246)
test bench::push_std ... bench: 42216 ns/iter (+/- 7903)

test result: ok. 0 passed; 0 failed; 1 ignored; 6 measured

% 

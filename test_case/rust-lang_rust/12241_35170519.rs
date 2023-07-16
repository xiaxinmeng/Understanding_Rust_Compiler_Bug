
$ rustc --opt-level=3 gistfile1.rs --test -Zlto && ./gistfile1 --bench is_utf8_100_ascii 

running 2 tests
test is_utf8_iter_inline::is_utf8_100_ascii ... bench:        95 ns/iter (+/- 2)
test is_utf8_ptr::is_utf8_100_ascii         ... bench:        63 ns/iter (+/- 1)

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured


$ cargo bench
    Finished release [optimized] target(s) in 0.00s
     Running target/release/deps/_14178-f02baafefc742e50

running 5 tests
test bench_builtin_ascii_uppercase                                                ... bench:          79 ns/iter (+/- 11)
test bench_custom_ascii_uppercase_with_ascii_check_builtin_is_ascii               ... bench:          76 ns/iter (+/- 13)
test bench_custom_ascii_uppercase_with_ascii_check_custom_is_ascii_convert_to_u32 ... bench:          78 ns/iter (+/- 8)
test bench_custom_ascii_uppercase_with_ascii_check_custom_is_ascii_convert_to_u8  ... bench:          80 ns/iter (+/- 24)
test bench_custom_ascii_uppercase_without_ascii_check                             ... bench:          58 ns/iter (+/- 9)

test result: ok. 0 passed; 0 failed; 0 ignored; 5 measured; 0 filtered out

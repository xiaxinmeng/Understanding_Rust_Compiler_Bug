
tartaros:rust ranma42$ rustc -O is_ascii.rs --test
tartaros:rust ranma42$ ./is_ascii --bench

running 22 tests
test is_ascii::byte_bench              ... bench:          65 ns/iter (+/- 6)
test is_ascii::char_bench              ... bench:          63 ns/iter (+/- 5)
test is_ascii_alphabetic::byte_bench   ... bench:          81 ns/iter (+/- 9)
test is_ascii_alphabetic::char_bench   ... bench:         125 ns/iter (+/- 23)
test is_ascii_alphanumeric::byte_bench ... bench:         114 ns/iter (+/- 8)
test is_ascii_alphanumeric::char_bench ... bench:         110 ns/iter (+/- 11)
test is_ascii_control::byte_bench      ... bench:          94 ns/iter (+/- 8)
test is_ascii_control::char_bench      ... bench:         105 ns/iter (+/- 50)
test is_ascii_digit::byte_bench        ... bench:          70 ns/iter (+/- 2)
test is_ascii_digit::char_bench        ... bench:          95 ns/iter (+/- 10)
test is_ascii_graphic::byte_bench      ... bench:          69 ns/iter (+/- 6)
test is_ascii_graphic::char_bench      ... bench:         100 ns/iter (+/- 3)
test is_ascii_hexdigit::byte_bench     ... bench:         118 ns/iter (+/- 10)
test is_ascii_hexdigit::char_bench     ... bench:         137 ns/iter (+/- 5)
test is_ascii_lowercase::byte_bench    ... bench:          70 ns/iter (+/- 10)
test is_ascii_lowercase::char_bench    ... bench:          94 ns/iter (+/- 11)
test is_ascii_punctuation::byte_bench  ... bench:         282 ns/iter (+/- 33)
test is_ascii_punctuation::char_bench  ... bench:         366 ns/iter (+/- 17)
test is_ascii_uppercase::byte_bench    ... bench:          70 ns/iter (+/- 4)
test is_ascii_uppercase::char_bench    ... bench:         111 ns/iter (+/- 16)
test is_ascii_whitespace::byte_bench   ... bench:          85 ns/iter (+/- 6)
test is_ascii_whitespace::char_bench   ... bench:          85 ns/iter (+/- 12)

test result: ok. 0 passed; 0 failed; 0 ignored; 22 measured; 0 filtered out

tartaros:rust ranma42$ ./rust/build/x86_64-apple-darwin/stage2/bin/rustc -O is_ascii.rs --test
tartaros:rust ranma42$ ./is_ascii --bench

running 22 tests
test is_ascii::byte_bench              ... bench:          63 ns/iter (+/- 8)
test is_ascii::char_bench              ... bench:          63 ns/iter (+/- 7)
test is_ascii_alphabetic::byte_bench   ... bench:          81 ns/iter (+/- 8)
test is_ascii_alphabetic::char_bench   ... bench:          81 ns/iter (+/- 7)
test is_ascii_alphanumeric::byte_bench ... bench:         116 ns/iter (+/- 8)
test is_ascii_alphanumeric::char_bench ... bench:         116 ns/iter (+/- 9)
test is_ascii_control::byte_bench      ... bench:          94 ns/iter (+/- 2)
test is_ascii_control::char_bench      ... bench:          93 ns/iter (+/- 7)
test is_ascii_digit::byte_bench        ... bench:          70 ns/iter (+/- 2)
test is_ascii_digit::char_bench        ... bench:          69 ns/iter (+/- 3)
test is_ascii_graphic::byte_bench      ... bench:          69 ns/iter (+/- 2)
test is_ascii_graphic::char_bench      ... bench:          70 ns/iter (+/- 2)
test is_ascii_hexdigit::byte_bench     ... bench:         135 ns/iter (+/- 21)
test is_ascii_hexdigit::char_bench     ... bench:         124 ns/iter (+/- 18)
test is_ascii_lowercase::byte_bench    ... bench:          73 ns/iter (+/- 5)
test is_ascii_lowercase::char_bench    ... bench:          70 ns/iter (+/- 2)
test is_ascii_punctuation::byte_bench  ... bench:         298 ns/iter (+/- 14)
test is_ascii_punctuation::char_bench  ... bench:         274 ns/iter (+/- 44)
test is_ascii_uppercase::byte_bench    ... bench:          69 ns/iter (+/- 7)
test is_ascii_uppercase::char_bench    ... bench:          69 ns/iter (+/- 5)
test is_ascii_whitespace::byte_bench   ... bench:          86 ns/iter (+/- 10)
test is_ascii_whitespace::char_bench   ... bench:          72 ns/iter (+/- 3)

test result: ok. 0 passed; 0 failed; 0 ignored; 22 measured; 0 filtered out

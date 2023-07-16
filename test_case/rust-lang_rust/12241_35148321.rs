
test is_utf8_iter::is_utf8_100_ascii            ... bench:       144 ns/iter (+/- 10)
test is_utf8_iter::is_utf8_100_multibyte        ... bench:       120 ns/iter (+/- 12)
test is_utf8_iter::random                       ... bench:       494 ns/iter (+/- 33)
test is_utf8_iter::random_invalid               ... bench:       276 ns/iter (+/- 443)

test is_utf8_iter_inline::is_utf8_100_ascii     ... bench:       116 ns/iter (+/- 1)
test is_utf8_iter_inline::is_utf8_100_multibyte ... bench:       110 ns/iter (+/- 3)
test is_utf8_iter_inline::random                ... bench:       493 ns/iter (+/- 34)
test is_utf8_iter_inline::random_invalid        ... bench:       249 ns/iter (+/- 400)

test is_utf8_ptr::is_utf8_100_ascii             ... bench:        88 ns/iter (+/- 5)
test is_utf8_ptr::is_utf8_100_multibyte         ... bench:        97 ns/iter (+/- 4)
test is_utf8_ptr::random                        ... bench:       369 ns/iter (+/- 24)
test is_utf8_ptr::random_invalid                ... bench:       239 ns/iter (+/- 342)

test is_utf8_std::is_utf8_100_ascii             ... bench:       122 ns/iter (+/- 4)
test is_utf8_std::is_utf8_100_multibyte         ... bench:       134 ns/iter (+/- 9)
test is_utf8_std::random                        ... bench:       512 ns/iter (+/- 43)
test is_utf8_std::random_invalid                ... bench:       264 ns/iter (+/- 430)

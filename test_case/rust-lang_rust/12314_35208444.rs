
new:
running 4 tests
test str::bench::from_utf8_lossy_100_ascii                      ... bench:        97 ns/iter (+/- 10)
test str::bench::from_utf8_lossy_100_invalid                    ... bench:       369 ns/iter (+/- 18)
test str::bench::from_utf8_lossy_100_multibyte                  ... bench:       103 ns/iter (+/- 3)
test str::bench::from_utf8_lossy_invalid                        ... bench:       102 ns/iter (+/- 8)

old:
running 4 tests
test str::bench::from_utf8_lossy_100_ascii                      ... bench:       112 ns/iter (+/- 2)
test str::bench::from_utf8_lossy_100_invalid                    ... bench:       384 ns/iter (+/- 17)
test str::bench::from_utf8_lossy_100_multibyte                  ... bench:       103 ns/iter (+/- 3)
test str::bench::from_utf8_lossy_invalid                        ... bench:       105 ns/iter (+/- 4)

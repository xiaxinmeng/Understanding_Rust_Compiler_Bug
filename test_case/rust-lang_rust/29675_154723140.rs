
before:

test t10000::libstd_sorted          ... bench:     429,275 ns/iter (+/- 1,836)
test t10000::libstd_sorted_10swaps  ... bench:     511,796 ns/iter (+/- 62,528)
test t10000::libstd_sorted_100swaps ... bench:     606,273 ns/iter (+/- 943)
test t10000::libstdsort             ... bench:     996,085 ns/iter (+/- 1,115)
test t1000::libstd_sorted           ... bench:      26,033 ns/iter (+/- 765)
test t1000::libstd_sorted_10swaps   ... bench:      35,167 ns/iter (+/- 507)
test t1000::libstd_sorted_100swaps  ... bench:      45,044 ns/iter (+/- 206)
test t1000::libstdsort              ... bench:      65,519 ns/iter (+/- 221)
test t100::libstd_sorted            ... bench:       1,842 ns/iter (+/- 9)
test t100::libstd_sorted_10swaps    ... bench:       2,675 ns/iter (+/- 20)
test t100::libstdsort               ... bench:       3,999 ns/iter (+/- 24)
test t10::libstd_sorted             ... bench:          74 ns/iter (+/- 1)
test t10::libstdsort                ... bench:         132 ns/iter (+/- 1)

after:

test t10000::libstd_sorted          ... bench:     175,627 ns/iter (+/- 747)
test t10000::libstd_sorted_10swaps  ... bench:     406,447 ns/iter (+/- 803)
test t10000::libstd_sorted_100swaps ... bench:     587,881 ns/iter (+/- 885)
test t10000::libstdsort             ... bench:     984,554 ns/iter (+/- 1,908)
test t1000::libstd_sorted           ... bench:      13,669 ns/iter (+/- 10)
test t1000::libstd_sorted_10swaps   ... bench:      34,950 ns/iter (+/- 90)
test t1000::libstd_sorted_100swaps  ... bench:      46,550 ns/iter (+/- 113)
test t1000::libstdsort              ... bench:      66,939 ns/iter (+/- 283)
test t100::libstd_sorted            ... bench:       5,450 ns/iter (+/- 10)
test t100::libstd_sorted_10swaps    ... bench:       6,798 ns/iter (+/- 9)
test t100::libstdsort               ... bench:       8,114 ns/iter (+/- 16)
test t10::libstd_sorted             ... bench:       2,204 ns/iter (+/- 4)
test t10::libstdsort                ... bench:       2,266 ns/iter (+/- 3)

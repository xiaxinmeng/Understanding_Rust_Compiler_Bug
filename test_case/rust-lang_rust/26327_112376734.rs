
commit 1:
test pathological_aa_100k::tw_contains                  ... bench:      16,289 ns/iter (+/- 18) = 6139 MB/s
test pathological_aa_100k::tw_contains_rev              ... bench:       7,196 ns/iter (+/- 197) = 13896 MB/s
test short_long::tw_contains                            ... bench:       7,733 ns/iter (+/- 100) = 329 MB/s
test short_long::tw_contains_rev                        ... bench:       5,910 ns/iter (+/- 41) = 431 MB/s
test short_short::tw_contains                           ... bench:         164 ns/iter (+/- 0) = 341 MB/s
test short_short::tw_contains_rev                       ... bench:         169 ns/iter (+/- 1) = 331 MB/s

commit 1 + 2:
test pathological_aa_100k::tw_contains                  ... bench:       3,533 ns/iter (+/- 420) = 28304 MB/s
test pathological_aa_100k::tw_contains_rev              ... bench:       3,840 ns/iter (+/- 252) = 26041 MB/s
test short_long::tw_contains                            ... bench:       3,322 ns/iter (+/- 18) = 767 MB/s
test short_long::tw_contains_rev                        ... bench:       3,343 ns/iter (+/- 44) = 763 MB/s
test short_short::tw_contains                           ... bench:         113 ns/iter (+/- 2) = 495 MB/s
test short_short::tw_contains_rev                       ... bench:         124 ns/iter (+/- 2) = 451 MB/s

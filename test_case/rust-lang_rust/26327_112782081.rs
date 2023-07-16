
before:
test bad_naive::tw_contains                             ... bench:         796 ns/iter (+/- 191) = 92 MB/s
test bad_naive_reversed::tw_contains                    ... bench:         297 ns/iter (+/- 9) = 249 MB/s
test pathological_aa_100k::tw_contains                  ... bench:       3,512 ns/iter (+/- 582) = 28473 MB/s
test pathological_aab_100k::tw_contains                 ... bench:   1,951,935 ns/iter (+/- 57,089) = 153 MB/s
test pathological_two_way_10k::tw_contains              ... bench:     241,923 ns/iter (+/- 4,562) = 123 MB/s
test pathological_two_way_20k::tw_contains              ... bench:     749,007 ns/iter (+/- 39,225) = 133 MB/s
test pathological_two_way_20k_reversed::tw_contains     ... bench:       3,216 ns/iter (+/- 55) = 18656 MB/s
test short_long::tw_contains                            ... bench:       3,338 ns/iter (+/- 16) = 764 MB/s
test short_short::tw_contains                           ... bench:         112 ns/iter (+/- 1) = 499 MB/s

after:
test bad_naive::tw_contains                             ... bench:         418 ns/iter (+/- 201) = 177 MB/s
test bad_naive_reversed::tw_contains                    ... bench:         263 ns/iter (+/- 22) = 281 MB/s
test pathological_aa_100k::tw_contains                  ... bench:       3,386 ns/iter (+/- 49) = 29533 MB/s
test pathological_aab_100k::tw_contains                 ... bench:   1,004,961 ns/iter (+/- 66,086) = 298 MB/s
test pathological_two_way_10k::tw_contains              ... bench:     114,635 ns/iter (+/- 28,467) = 261 MB/s
test pathological_two_way_20k::tw_contains              ... bench:     362,096 ns/iter (+/- 34,481) = 276 MB/s
test pathological_two_way_20k_reversed::tw_contains     ... bench:       3,209 ns/iter (+/- 33) = 18697 MB/s
test short_long::tw_contains                            ... bench:       2,398 ns/iter (+/- 665) = 1063 MB/s
test short_short::tw_contains                           ... bench:         102 ns/iter (+/- 6) = 549 MB/s

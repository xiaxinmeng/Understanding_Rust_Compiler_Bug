
# actually_monotonic() == true
test time::tests::instant_contention_01_threads                   ... bench:          44 ns/iter (+/- 0)
test time::tests::instant_contention_02_threads                   ... bench:          44 ns/iter (+/- 0)
test time::tests::instant_contention_04_threads                   ... bench:          44 ns/iter (+/- 0)
test time::tests::instant_contention_08_threads                   ... bench:          44 ns/iter (+/- 0)
test time::tests::instant_contention_16_threads                   ... bench:          44 ns/iter (+/- 0)

# cmpxchg16b
test time::tests::instant_contention_01_threads                   ... bench:          56 ns/iter (+/- 1)
test time::tests::instant_contention_02_threads                   ... bench:         155 ns/iter (+/- 0)
test time::tests::instant_contention_04_threads                   ... bench:         351 ns/iter (+/- 3)
test time::tests::instant_contention_08_threads                   ... bench:         747 ns/iter (+/- 15)
test time::tests::instant_contention_16_threads                   ... bench:       1,084 ns/iter (+/- 17)

# 2x AtomicU64
test time::tests::instant_contention_01_threads                   ... bench:          74 ns/iter (+/- 0)
test time::tests::instant_contention_02_threads                   ... bench:         268 ns/iter (+/- 75)
test time::tests::instant_contention_04_threads                   ... bench:         481 ns/iter (+/- 68)
test time::tests::instant_contention_08_threads                   ... bench:         829 ns/iter (+/- 51)
test time::tests::instant_contention_16_threads                   ... bench:       1,286 ns/iter (+/- 23)

# mutex
test time::tests::instant_contention_01_threads                   ... bench:          60 ns/iter (+/- 2)
test time::tests::instant_contention_02_threads                   ... bench:         770 ns/iter (+/- 231)
test time::tests::instant_contention_04_threads                   ... bench:       1,347 ns/iter (+/- 45)
test time::tests::instant_contention_08_threads                   ... bench:       2,693 ns/iter (+/- 114)
test time::tests::instant_contention_16_threads                   ... bench:       5,244 ns/iter (+/- 487)

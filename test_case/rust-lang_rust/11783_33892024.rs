
test siphash::chunks_01       ... bench:    793589 ns/iter (+/- 53379) = 82 MB/s
test xxhash::clang::chunks_01 ... bench:   3517378 ns/iter (+/- 7811) = 74 MB/s
test xxhash::rust::chunks_01  ... bench:    851273 ns/iter (+/- 1006) = 76 MB/s

test siphash::chunks_02       ... bench:    521734 ns/iter (+/- 6753) = 125 MB/s
test xxhash::clang::chunks_02 ... bench:   2072373 ns/iter (+/- 2239) = 126 MB/s
test xxhash::rust::chunks_02  ... bench:    480232 ns/iter (+/- 3435) = 136 MB/s

test siphash::chunks_04       ... bench:    380421 ns/iter (+/- 77124) = 172 MB/s
test xxhash::clang::chunks_04 ... bench:   1267936 ns/iter (+/- 3077) = 206 MB/s
test xxhash::rust::chunks_04  ... bench:    241673 ns/iter (+/- 1050) = 271 MB/s

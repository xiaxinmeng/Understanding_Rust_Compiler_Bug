rust
6830 bytes string:

alloc_only                ... bench:    109 ns/iter (+/- 0) = 62660 MB/s
black_box_read_each_byte  ... bench:  1,708 ns/iter (+/- 5) = 3998 MB/s
lookup                    ... bench:  1,725 ns/iter (+/- 2) = 3959 MB/s
branch_and_subtract       ... bench:    413 ns/iter (+/- 1) = 16537 MB/s
branch_and_mask           ... bench:    411 ns/iter (+/- 2) = 16618 MB/s
branchless                ... bench:    377 ns/iter (+/- 2) = 18116 MB/s
libcore                   ... bench:    378 ns/iter (+/- 2) = 18068 MB/s
fake_simd_u32             ... bench:    373 ns/iter (+/- 1) = 18310 MB/s
fake_simd_u64             ... bench:    374 ns/iter (+/- 0) = 18262 MB/s

32 bytes string:

alloc_only                ... bench:     13 ns/iter (+/- 0) = 2461 MB/s
black_box_read_each_byte  ... bench:     28 ns/iter (+/- 0) = 1142 MB/s
lookup                    ... bench:     25 ns/iter (+/- 0) = 1280 MB/s
branch_and_subtract       ... bench:     16 ns/iter (+/- 0) = 2000 MB/s
branch_and_mask           ... bench:     16 ns/iter (+/- 0) = 2000 MB/s
branchless                ... bench:     15 ns/iter (+/- 0) = 2133 MB/s
libcore                   ... bench:     16 ns/iter (+/- 0) = 2000 MB/s
fake_simd_u32             ... bench:     17 ns/iter (+/- 0) = 1882 MB/s
fake_simd_u64             ... bench:     17 ns/iter (+/- 0) = 1882 MB/s

7 bytes string:

alloc_only                ... bench:     13 ns/iter (+/- 0) = 538 MB/s
black_box_read_each_byte  ... bench:     22 ns/iter (+/- 0) = 318 MB/s
lookup                    ... bench:     17 ns/iter (+/- 0) = 411 MB/s
branch_and_subtract       ... bench:     16 ns/iter (+/- 0) = 437 MB/s
branch_and_mask           ... bench:     17 ns/iter (+/- 0) = 411 MB/s
branchless                ... bench:     21 ns/iter (+/- 0) = 333 MB/s
libcore                   ... bench:     21 ns/iter (+/- 0) = 333 MB/s
fake_simd_u32             ... bench:     20 ns/iter (+/- 0) = 350 MB/s
fake_simd_u64             ... bench:     23 ns/iter (+/- 0) = 304 MB/s

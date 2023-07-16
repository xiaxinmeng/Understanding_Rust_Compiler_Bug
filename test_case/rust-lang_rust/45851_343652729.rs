rust
     Running target/release/deps/read_to_end_bench-9122fdf004b0166c

running 24 tests
test a_read_128::vec_new            ... bench:       1,380 ns/iter (+/- 49) = 92 MB/s
test a_read_128::vec_with_capacity  ... bench:       1,416 ns/iter (+/- 107) = 90 MB/s
test b_read_512::vec_new            ... bench:       1,690 ns/iter (+/- 78) = 302 MB/s
test b_read_512::vec_with_capacity  ... bench:       1,736 ns/iter (+/- 52) = 294 MB/s
test c_read_2k::vec_new             ... bench:       2,085 ns/iter (+/- 113) = 982 MB/s
test c_read_2k::vec_with_capacity   ... bench:       2,015 ns/iter (+/- 72) = 1016 MB/s
test d_read_8k::vec_new             ... bench:       2,778 ns/iter (+/- 131) = 2948 MB/s
test d_read_8k::vec_with_capacity   ... bench:       2,516 ns/iter (+/- 89) = 3255 MB/s
test e_read_32k::vec_new            ... bench:       5,107 ns/iter (+/- 103) = 6416 MB/s
test e_read_32k::vec_with_capacity  ... bench:       4,404 ns/iter (+/- 81) = 7440 MB/s
test f_read_128k::vec_new           ... bench:      16,232 ns/iter (+/- 106) = 8074 MB/s
test f_read_128k::vec_with_capacity ... bench:      12,223 ns/iter (+/- 103) = 10723 MB/s
test g_read_512k::vec_new           ... bench:      39,179 ns/iter (+/- 210) = 13381 MB/s
test g_read_512k::vec_with_capacity ... bench:      31,704 ns/iter (+/- 98) = 16536 MB/s
test h_read_1m::vec_new             ... bench:     463,119 ns/iter (+/- 2,354) = 2264 MB/s
test h_read_1m::vec_with_capacity   ... bench:     251,983 ns/iter (+/- 3,072) = 4161 MB/s
test i_read_2m::vec_new             ... bench:     668,742 ns/iter (+/- 8,317) = 3135 MB/s
test i_read_2m::vec_with_capacity   ... bench:     383,879 ns/iter (+/- 1,269) = 5463 MB/s
test j_read_4m::vec_new             ... bench:   1,188,553 ns/iter (+/- 13,409) = 3528 MB/s
test j_read_4m::vec_with_capacity   ... bench:     857,714 ns/iter (+/- 8,254) = 4890 MB/s
test k_read_8m::vec_new             ... bench:   2,302,201 ns/iter (+/- 15,746) = 3643 MB/s
test k_read_8m::vec_with_capacity   ... bench:   1,947,089 ns/iter (+/- 10,942) = 4308 MB/s
test l_read_32m::vec_new            ... bench:   8,990,230 ns/iter (+/- 18,718) = 3732 MB/s
test l_read_32m::vec_with_capacity  ... bench:   8,648,669 ns/iter (+/- 23,157) = 3879 MB/s

test result: ok. 0 passed; 0 failed; 0 ignored; 24 measured; 0 filtered out

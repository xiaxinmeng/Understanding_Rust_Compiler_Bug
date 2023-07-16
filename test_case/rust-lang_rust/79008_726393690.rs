
# userspace copy

test io::tests::bench_file_to_file_copy         ... bench:      21,002 ns/iter (+/- 750) = 6240 MB/s    [ext4]
test io::tests::bench_file_to_file_copy         ... bench:      35,704 ns/iter (+/- 1,108) = 3671 MB/s  [btrfs]

# copy_file_range

test io::tests::bench_file_to_file_copy         ... bench:      14,745 ns/iter (+/- 519) = 8889 MB/s    [ext4]
test io::tests::bench_file_to_file_copy         ... bench:       6,128 ns/iter (+/- 227) = 21389 MB/s   [btrfs]

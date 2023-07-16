
     Running unittests (target\release\deps\bench_u32s-967e14898f5691eb.exe)

Gnuplot not found, using plotters backend
cmp/u32s struct/Self    time:   [6.7752 us 6.7880 us 6.8033 us]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
cmp/u32s struct/Random field
                        time:   [15.176 us 15.204 us 15.234 us]
Found 10 outliers among 100 measurements (10.00%)
  9 (9.00%) high mild
  1 (1.00%) high severe
cmp/u32s struct/First field
                        time:   [2.6016 us 2.6084 us 2.6149 us]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
cmp/u32s struct/Last field
                        time:   [6.7269 us 6.7357 us 6.7459 us]
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe
cmp/u32s struct/Every Second
                        time:   [12.925 us 12.940 us 12.957 us]
Found 18 outliers among 100 measurements (18.00%)
  18 (18.00%) high severe

     Running unittests (target\release\deps\bench_with_strings-77b5d3b11469681f.exe)

Gnuplot not found, using plotters backend
cmp/String struct/Self  time:   [65.674 us 65.750 us 65.823 us]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
cmp/String struct/u32 field
                        time:   [17.712 us 17.733 us 17.756 us]
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild
cmp/String struct/String field
                        time:   [10.087 us 10.102 us 10.116 us]
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) low mild
  5 (5.00%) high mild

shell
sort_unstable/length: 25, modulus: 5
                        time:   [244.74 ns 245.26 ns 245.77 ns]
                        change: [+3.3725% +3.7220% +4.0764%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild

sort_unstable/length: 25, modulus: 10
                        time:   [164.97 ns 165.27 ns 165.57 ns]
                        change: [-2.1635% -1.8905% -1.6316%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

sort_unstable/length: 25, modulus: 100
                        time:   [185.64 ns 185.97 ns 186.30 ns]
                        change: [-4.3990% -4.0341% -3.6860%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild

sort_unstable/length: 25, modulus: 1000
                        time:   [241.00 ns 242.61 ns 244.56 ns]
                        change: [-4.6262% -4.1790% -3.6419%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

sort_unstable/length: 500, modulus: 5
                        time:   [3.4800 us 3.4832 us 3.4866 us]
                        change: [+0.3840% +0.6176% +0.8530%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe

sort_unstable/length: 500, modulus: 10
                        time:   [5.1528 us 5.1832 us 5.2246 us]
                        change: [+2.7131% +3.3381% +4.1485%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) high mild
  6 (6.00%) high severe

sort_unstable/length: 500, modulus: 100
                        time:   [8.2492 us 8.2640 us 8.2817 us]
                        change: [-1.3892% -1.0490% -0.7131%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe

sort_unstable/length: 500, modulus: 1000
                        time:   [8.5341 us 8.5413 us 8.5493 us]
                        change: [-0.6444% -0.4038% -0.1390%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe

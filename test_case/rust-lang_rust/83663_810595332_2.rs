
     Running unittests (target\release\deps\bench_u32s-967e14898f5691eb.exe)
Gnuplot not found, using plotters backend
cmp/u32s struct/Self    time:   [7.1867 us 7.2374 us 7.2920 us]
                        change: [+6.1698% +7.0979% +7.9865%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild
cmp/u32s struct/Random field
                        time:   [6.7970 us 6.8126 us 6.8319 us]
                        change: [-54.664% -53.917% -53.142%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe
cmp/u32s struct/First field
                        time:   [6.7780 us 6.7868 us 6.7966 us]
                        change: [+158.65% +159.32% +160.07%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) low mild
  1 (1.00%) high severe
cmp/u32s struct/Last field
                        time:   [6.7810 us 6.7919 us 6.8034 us]
                        change: [+0.3407% +0.6019% +0.9169%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) low mild
  1 (1.00%) high mild
  1 (1.00%) high severe
cmp/u32s struct/Every Second
                        time:   [6.7663 us 6.7758 us 6.7861 us]
                        change: [-48.465% -48.193% -47.936%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild

     Running unittests (target\release\deps\bench_with_strings-77b5d3b11469681f.exe)

Gnuplot not found, using plotters backend
cmp/String struct/Self  time:   [59.658 us 59.743 us 59.833 us]
                        change: [-9.1402% -8.9131% -8.6605%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  3 (3.00%) low severe
  3 (3.00%) low mild
  5 (5.00%) high mild
  1 (1.00%) high severe
cmp/String struct/u32 field
                        time:   [3.9043 us 3.9140 us 3.9253 us]
                        change: [-77.864% -77.797% -77.726%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe
cmp/String struct/String field
                        time:   [6.9662 us 6.9746 us 6.9833 us]
                        change: [-31.134% -30.958% -30.788%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe

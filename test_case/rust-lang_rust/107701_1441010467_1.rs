
 name                                  base.b ns/iter  branch.b ns/iter  diff ns/iter   diff %  speedup 
 iter::bench_chain_partial_cmp         101,526         98,963                  -2,563   -2.52%   x 1.03 
 iter::bench_enumerate_chain_ref_sum   3,819,937       2,088,350           -1,731,587  -45.33%   x 1.83 
 iter::bench_enumerate_chain_sum       909,045         932,297                 23,252    2.56%   x 0.98 
 iter::bench_filter_chain_count        1,072,605       953,229               -119,376  -11.13%   x 1.13 
 iter::bench_filter_chain_ref_count    3,963,479       2,322,925           -1,640,554  -41.39%   x 1.71 
 iter::bench_filter_chain_ref_sum      1,711,480       1,612,636              -98,844   -5.78%   x 1.06 
 iter::bench_filter_chain_sum          1,258,180       1,135,219             -122,961   -9.77%   x 1.11 
 iter::bench_filter_map_chain_ref_sum  2,146,493       2,223,921               77,428    3.61%   x 0.97 
 iter::bench_flat_map_chain_ref_sum    4,753,742       6,936,048            2,182,306   45.91%   x 0.69 
 iter::bench_for_each_chain_fold       472,987         695,519                222,532   47.05%   x 0.68 
 iter::bench_for_each_chain_loop       3,872,177       1,856,980           -2,015,197  -52.04%   x 2.09 
 iter::bench_for_each_chain_ref_fold   3,907,860       2,009,307           -1,898,553  -48.58%   x 1.94 
 iter::bench_fuse_chain_ref_sum        3,814,276       2,320,439           -1,493,837  -39.16%   x 1.64 
 iter::bench_inspect_chain_ref_sum     3,889,967       2,010,567           -1,879,400  -48.31%   x 1.93 
 iter::bench_inspect_chain_sum         473,616         463,739                 -9,877   -2.09%   x 1.02 
 iter::bench_peekable_chain_ref_sum    3,902,686       2,012,245           -1,890,441  -48.44%   x 1.94 
 iter::bench_peekable_chain_sum        472,299         694,289                221,990   47.00%   x 0.68 
 iter::bench_skip_chain_ref_sum        3,909,425       2,083,966           -1,825,459  -46.69%   x 1.88 
 iter::bench_skip_chain_sum            709,471         463,708               -245,763  -34.64%   x 1.53 
 iter::bench_skip_while_chain_sum      473,519         695,629                222,110   46.91%   x 0.68 
 iter::bench_slice_chain_ref_sum       573             426                       -147  -25.65%   x 1.35 
 iter::bench_slice_chain_sum           534             517                        -17   -3.18%   x 1.03 
 iter::bench_take_while_chain_ref_sum  924,697         1,056,233              131,536   14.22%   x 0.88 
 iter::bench_take_while_chain_sum      263,289         500,973                237,684   90.27%   x 0.53 

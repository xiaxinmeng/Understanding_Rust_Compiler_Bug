
# RUSTFLAGS="-Ccodegen-units=1 -Copt-level=3" taskset -c 5 schedtool -B -e ./x.py bench --stage 0 library/core/ --test-args "cycle"

 iter::bench_cycle_skip_take_ref_sum          1,837,376         1,110,352              -727,024  -39.57%   x 1.65 
 iter::bench_cycle_skip_take_sum              1,297,196         525,064                -772,132  -59.52%   x 2.47 
 iter::bench_cycle_take_ref_sum               1,096,240         1,093,491                -2,749   -0.25%   x 1.00 
 iter::bench_cycle_take_skip_ref_sum          588,162           717,214                 129,052   21.94%   x 0.82 
 iter::bench_cycle_take_skip_sum              550,002           474,185                 -75,817  -13.78%   x 1.16 
 iter::bench_cycle_take_sum                   524,600           448,794                 -75,806  -14.45%   x 1.17 
 iter::bench_skip_cycle_skip_zip_add_ref_sum  5,336,791         3,761,926            -1,574,865  -29.51%   x 1.42 
 iter::bench_skip_cycle_skip_zip_add_sum      4,075,445         4,038,599               -36,846   -0.90%   x 1.01 

# RUSTFLAGS="-Ccodegen-units=1 -Copt-level=2 -Ctarget-cpu=native" taskset -c 5 schedtool -B -e ./x.py bench --stage 0 library/core/ --test-args "cycle"

 iter::bench_cycle_skip_take_ref_sum          1,396,256         1,087,191              -309,065  -22.14%   x 1.28 
 iter::bench_cycle_skip_take_sum              1,260,624         474,665                -785,959  -62.35%   x 2.66 
 iter::bench_cycle_take_ref_sum               1,095,467         1,094,409                -1,058   -0.10%   x 1.00 
 iter::bench_cycle_take_skip_ref_sum          1,058,240         1,008,308               -49,932   -4.72%   x 1.05 
 iter::bench_cycle_take_skip_sum              496,065           448,904                 -47,161   -9.51%   x 1.11 
 iter::bench_cycle_take_sum                   469,290           466,544                  -2,746   -0.59%   x 1.01 
 iter::bench_skip_cycle_skip_zip_add_ref_sum  3,453,381         3,737,632               284,251    8.23%   x 0.92 
 iter::bench_skip_cycle_skip_zip_add_sum      3,179,488         3,472,014               292,526    9.20%   x 0.92 


[BENCH RUN] ebobby/simple-raytracer
Benchmark #1: ./raytracer_cg_clif_no_zero_pad
  Time (mean ± σ):     13.009 s ±  0.853 s    [User: 12.964 s, System: 0.016 s]
  Range (min … max):   12.061 s … 14.689 s    10 runs
 
Benchmark #2: ./raytracer_cg_clif_release
  Time (mean ± σ):     10.194 s ±  0.454 s    [User: 10.162 s, System: 0.012 s]
  Range (min … max):    9.523 s … 11.052 s    10 runs
 
Benchmark #3: ./raytracer_cg_clif_zero_pad
  Time (mean ± σ):     12.320 s ±  0.416 s    [User: 12.302 s, System: 0.006 s]
  Range (min … max):   11.850 s … 13.185 s    10 runs
 
Benchmark #4: ./raytracer_cg_clif_zero_pad_arbitrary
  Time (mean ± σ):     12.618 s ±  0.684 s    [User: 12.583 s, System: 0.011 s]
  Range (min … max):   11.933 s … 13.932 s    10 runs
 
Benchmark #5: ./raytracer_cg_clif_zero_pad_prologue
  Time (mean ± σ):     11.997 s ±  0.076 s    [User: 11.976 s, System: 0.006 s]
  Range (min … max):   11.906 s … 12.155 s    10 runs
 
Benchmark #6: ./raytracer_cg_clif_zero_pad_prologue_enum_too
  Time (mean ± σ):     12.205 s ±  0.152 s    [User: 12.192 s, System: 0.006 s]
  Range (min … max):   12.096 s … 12.574 s    10 runs
 
Benchmark #7: ./raytracer_cg_clif_zero_pad_prologue_enum_too_release
  Time (mean ± σ):      9.440 s ±  0.081 s    [User: 9.425 s, System: 0.007 s]
  Range (min … max):    9.374 s …  9.656 s    10 runs
 
Summary
  './raytracer_cg_clif_zero_pad_prologue_enum_too_release' ran
    1.08 ± 0.05 times faster than './raytracer_cg_clif_release'
    1.27 ± 0.01 times faster than './raytracer_cg_clif_zero_pad_prologue'
    1.29 ± 0.02 times faster than './raytracer_cg_clif_zero_pad_prologue_enum_too'
    1.31 ± 0.05 times faster than './raytracer_cg_clif_zero_pad'
    1.34 ± 0.07 times faster than './raytracer_cg_clif_zero_pad_arbitrary'
    1.38 ± 0.09 times faster than './raytracer_cg_clif_no_zero_pad'


$ hyperfine -L rev dfb40c3d0b34f2ad4bc72b4cf8645cd872928be2,005fc0f00f2d4ceaf523b67a8f9c5665b8ac5baf -s "git checkout {rev};x test tidy;x clean" -w 2 -m 5 -n "x test tidy of {rev}" "x test tidy"

Benchmark 1: x test tidy of dfb40c3d0b34f2ad4bc72b4cf8645cd872928be2
  Time (mean ± σ):     14.397 s ±  0.053 s    [User: 30.262 s, System: 3.714 s]
  Range (min … max):   14.328 s … 14.473 s    5 runs

Benchmark 2: x test tidy of 005fc0f00f2d4ceaf523b67a8f9c5665b8ac5baf
  Time (mean ± σ):      2.178 s ±  0.033 s    [User: 18.719 s, System: 2.748 s]
  Range (min … max):    2.145 s …  2.219 s    5 runs

Summary
  'x test tidy of 005fc0f00f2d4ceaf523b67a8f9c5665b8ac5baf' ran
    6.61 ± 0.10 times faster than 'x test tidy of dfb40c3d0b34f2ad4bc72b4cf8645cd872928be2'

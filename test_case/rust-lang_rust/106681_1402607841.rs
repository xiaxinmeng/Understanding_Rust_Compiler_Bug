
$ hyperfine -L rev dfb40c3d0b34f2ad4bc72b4cf8645cd872928be2,005fc0f00f2d4ceaf523b67a8f9c5665b8ac5baf -s "git checkout {rev};x test tidy;x clean" -w 2 -p "x clean" -m 5 -n "x test tidy of {rev}" "x test tidy"

Benchmark 1: x test tidy of dfb40c3d0b34f2ad4bc72b4cf8645cd872928be2
  Time (mean ± σ):     83.100 s ±  0.655 s    [User: 257.193 s, System: 23.844 s]
  Range (min … max):   82.476 s … 84.087 s    5 runs

Benchmark 2: x test tidy of 005fc0f00f2d4ceaf523b67a8f9c5665b8ac5baf
  Time (mean ± σ):     48.405 s ±  0.576 s    [User: 171.333 s, System: 18.779 s]
  Range (min … max):   47.721 s … 49.040 s    5 runs

Summary
  'x test tidy of 005fc0f00f2d4ceaf523b67a8f9c5665b8ac5baf' ran
    1.72 ± 0.02 times faster than 'x test tidy of dfb40c3d0b34f2ad4bc72b4cf8645cd872928be2'

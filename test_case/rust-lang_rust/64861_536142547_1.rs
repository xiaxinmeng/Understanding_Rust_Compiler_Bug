
$ hyperfine './old' './new'
Benchmark #1: ./old
  Time (mean ± σ):     462.1 ms ±   4.4 ms    [User: 241.5 ms, System: 218.2 ms]
  Range (min … max):   453.9 ms … 467.4 ms    10 runs
 
Benchmark #2: ./new
  Time (mean ± σ):     109.9 ms ±   5.5 ms    [User: 107.9 ms, System: 1.3 ms]
  Range (min … max):   102.0 ms … 120.6 ms    27 runs
 
Summary
  './new' ran
    4.20 ± 0.21 times faster than './old'

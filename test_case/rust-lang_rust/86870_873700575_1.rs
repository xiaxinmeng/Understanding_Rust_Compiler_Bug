text
$ hyperfine 'clang++ -O2 test_500.cpp -Wno-xor-used-as-pow'
Benchmark #1: clang++ -O2 test_500.cpp -Wno-xor-used-as-pow
  Time (mean ± σ):      2.347 s ±  0.079 s    [User: 2.271 s, System: 0.053 s]
  Range (min … max):    2.249 s …  2.462 s    10 runs

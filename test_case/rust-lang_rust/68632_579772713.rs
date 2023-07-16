
running 2 tests
tests::benchmark_bad  ... bench:   3,293,805 ns/iter (+/- 17,375)
tests::benchmark_good ... bench:   3,309,870 ns/iter (+/- 46,277)                                                                                                                                                                    
test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured; 0 filtered out                                                                                                                                                                
C:\Users\idkra\dev\rust\issue-68632>cargo bench                                                                         
Compiling issue-68632 v0.1.0 (C:\Users\idkra\dev\rust\issue-68632)
    Finished bench [optimized] target(s) in 0.51s
     Running target\release\deps\issue_68632-80a8ccd9df83eb66.exe

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target\release\deps\bench_builtin-30e90d201571b52f.exe

running 2 tests
test tests::benchmark_bad  ... bench:   3,375,950 ns/iter (+/- 32,356)
test tests::benchmark_good ... bench:   2,596,490 ns/iter (+/- 14,938)

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured; 0 filtered out

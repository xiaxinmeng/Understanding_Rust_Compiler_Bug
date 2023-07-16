
listless% echo stable > rust-toolchain                                        
listless% hyperfine rustc                                                     
Benchmark #1: rustc
  Time (mean ± σ):      69.8 ms ±   1.9 ms    [User: 59.5 ms, System: 10.2 ms]
  Range (min … max):    67.4 ms …  76.1 ms    42 runs
 
listless% echo beta > rust-toolchain                                          
listless% hyperfine rustc                                                    
Benchmark #1: rustc
  Time (mean ± σ):      19.6 ms ±   0.7 ms    [User: 14.2 ms, System: 5.4 ms]
  Range (min … max):    18.7 ms …  22.9 ms    128 runs
 
listless%                                                                    

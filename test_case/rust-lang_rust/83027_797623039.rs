
$ lscpu | rg Model
Model:                           79
Model name:                      Intel(R) Core(TM) i7-6900K CPU @ 3.20GHz
$ rustc --print target-cpus | rg native
    native         - Select the CPU of the current host (currently broadwell).

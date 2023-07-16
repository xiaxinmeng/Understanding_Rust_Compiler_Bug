
# using cmath
$ rustc --opt-level=3 intrinsics-bench.rs  && time ./intrinsics-bench 
666666661666.567017

real    0m2.729s
user    0m2.712s
sys     0m0.008s

# using intrinsics
$ x86_64-unknown-linux-gnu/stage2/bin/rustc --opt-level=3 intrinsics-bench.rs  && time ./intrinsics-bench 
666666661666.567017

real    0m0.513s
user    0m0.500s
sys     0m0.008s

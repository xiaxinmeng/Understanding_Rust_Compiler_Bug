
> rustc -Z time-passes hello-world.rs | grep -v '0.000'
time: 0.002     parsing
time: 0.042     plugin loading
time: 0.023     expansion
time: 0.001     assigning node ids and indexing ast
time: 0.063     resolution
time: 0.001     type collecting
time: 1.166     coherence checking
time: 0.015     type checking
time: 0.020     check static items
time: 0.032     borrow checking
time: 0.005     rvalue checking
time: 0.001     lint checking
time: 0.083     translation
  time: 0.003   llvm function passes
  time: 0.002   llvm module passes
  time: 0.056   codegen passes
  time: 0.005   codegen passes
time: 0.112     LLVM passes
  time: 3.648   running linker
time: 3.656     linking
rustc -Z time-passes hello-world.rs  4.83s user 0.69s system 98% cpu 5.588 total

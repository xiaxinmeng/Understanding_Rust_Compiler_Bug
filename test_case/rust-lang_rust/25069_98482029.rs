
$ rustc --version
rustc 1.1.0-nightly (c42c1e7a6 2015-05-02) (built 2015-05-01)
$ time rustc --crate-type=lib -Z no-trans inflate.rs

real    0m19.984s
user    0m18.948s
sys 0m1.033s
$ rustc --crate-type=lib -Z no-trans -Z time-passes inflate.rs
time: 0.007     parsing
time: 0.000     recursion limit
time: 0.001     configuration 1
time: 0.000     gated macro checking
time: 0.000     crate injection
time: 0.009     macro loading
time: 0.000     plugin loading
time: 0.000     plugin registration
time: 0.169     expansion
time: 0.001     complete gated feature checking 1
time: 0.018     configuration 2
time: 0.009     maybe building test harness
time: 0.008     prelude injection
time: 0.001     checking that all macro invocations are gone
time: 0.001     complete gated feature checking 2
time: 0.010     assigning node ids and indexing ast
time: 0.001     external crate/lib resolution
time: 0.002     language item collection
time: 0.037     resolution
time: 0.002     lifetime resolution
time: 0.000     looking for entry point
time: 0.000     looking for plugin registrar
time: 0.012     region resolution
time: 0.001     loop checking
time: 0.001     static item recursion checking
time: 0.002     type collecting
time: 0.001     variance inference
time: 0.129     coherence checking
time: 18.775    type checking
time: 0.048     const checking
time: 0.009     privacy checking
time: 0.000     stability index
time: 0.003     intrinsic checking
time: 0.003     effect checking
time: 0.006     match checking
time: 0.194     liveness checking
time: 0.438     borrow checking
time: 0.035     rvalue checking
time: 0.000     reachability checking
time: 0.007     death checking
time: 0.079     stability checking
time: 0.000     unused lib feature checking
time: 0.116     lint checking

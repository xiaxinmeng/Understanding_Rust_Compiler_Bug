error: unsatisfied lifetime constraints=========================>    ] 120/128: rustc_mir                                                                                                                                                    
  --> src/librustc_mir/interpret/snapshot.rs:75:34
   |
46 | impl<'a, 'mir, 'tcx> InfiniteLoopDetector<'a, 'mir, 'tcx>
   |      --  ---- lifetime `'mir` defined here
   |      |
   |      lifetime `'a` defined here
...
75 |         if self.snapshots.insert(EvalSnapshot::new(memory, stack)) {
   |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'mir` must outlive `'a`

error: unsatisfied lifetime constraints
   --> src/librustc_mir/interpret/snapshot.rs:410:1
    |
410 |   impl_stable_hash_for!(impl<'tcx, 'b, 'mir> for struct EvalSnapshot<'b, 'mir, 'tcx> {
    |   ^                          ----      ---- lifetime `'mir` defined here
    |   |                          |
    |  _|                          lifetime `'tcx` defined here
    | |
411 | |     // Not hashing memory: Avoid hashing memory all the time during execution
412 | |     memory -> _,
413 | |     stack,
414 | | });
    | |___^ type annotation requires that `'mir` must outlive `'tcx`
    |
    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

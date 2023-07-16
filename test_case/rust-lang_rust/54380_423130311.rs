plain
[00:13:31]    Compiling rustc_metadata_utils v0.0.0 (file:///checkout/src/librustc_metadata_utils)
[00:13:31]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:13:31]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:13:31]    Compiling rustc_traits v0.0.0 (file:///checkout/src/librustc_traits)
[00:14:20] error[E0502]: cannot borrow `*ecx` as immutable because `ecx.machine.loop_detector` is also borrowed as mutable
[00:14:20]     |
[00:14:20]     |
[00:14:20] 365 |         ecx.machine.loop_detector.observe_and_analyze(
[00:14:20]     |         ------------------------- mutable borrow occurs here
[00:14:20] 366 |             &ecx.tcx,
[00:14:20] 367 |             ecx.frame().span,
[00:14:20] ...
[00:14:20] 370 |         )
[00:14:20] 370 |         )
[00:14:20]     |         - mutable borrow ends here
[00:14:20] error: aborting due to previous error
[00:14:20] 
[00:14:20] For more information about this error, try `rustc --explain E0502`.
[00:14:20] error: Could not compile `rustc_mir`.

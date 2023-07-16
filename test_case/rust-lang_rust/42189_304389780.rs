
[01:08:42] diff of stderr:
[01:08:42] 
[01:08:42]  error[E0391]: unsupported cyclic reference between types/traits detected
[01:08:42]    |
[01:08:42]  note: the cycle begins when computing layout of `S`...
[01:08:42]  note: ...which then requires computing layout of `std::option::Option<<S as Mirror>::It>`...
[01:08:42]  note: ...which then requires computing layout of `<S as Mirror>::It`...
[01:08:42]    = note: ...which then again requires computing layout of `S`, completing the cycle.
[01:08:42]  
[01:08:42] -error: aborting due to previous error
[01:08:42] +error: aborting due to previous error(s)
[01:08:42]  
[01:08:42] 
[01:08:42] The actual stderr differed from the expected stderr.
[01:08:42] 
[01:08:42] ------------------------------------------
[01:08:42] stderr:
[01:08:42] ------------------------------------------
[01:08:42] error[E0391]: unsupported cyclic reference between types/traits detected
[01:08:42]   |
[01:08:42] note: the cycle begins when computing layout of `S`...
[01:08:42] note: ...which then requires computing layout of `std::option::Option<<S as Mirror>::It>`...
[01:08:42] note: ...which then requires computing layout of `<S as Mirror>::It`...
[01:08:42]   = note: ...which then again requires computing layout of `S`, completing the cycle.
[01:08:42] 
[01:08:42] error: aborting due to previous error(s)
[01:08:42] 
[01:08:42] ------------------------------------------
[01:08:42] 
[01:08:42] thread '[ui] ui/issue-26548.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2472
[01:08:42] failures:
[01:08:42]     [ui] ui/issue-26548.rs

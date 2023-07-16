
iss_17539.rs:1:23: 1:29 error: unsupported cyclic reference between types/traits detected
iss_17539.rs:1 type SomeFn = fn() -> SomeFn;
                                     ^~~~~~
note: the cycle begins when processing `SomeFn`...
note: ...which then again requires processing `SomeFn`, completing the cycle.
error: aborting due to previous error

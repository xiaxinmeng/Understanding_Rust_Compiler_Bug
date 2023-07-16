 shell
$ rustc main.rs
main.rs:1:1: 2:2 error: unsupported cyclic reference between types/traits detected
main.rs:1 trait t1 : t2 {
main.rs:2 }
note: the cycle begins when computing the supertraits of `t1`...
note: ...which then requires computing the supertraits of `t2`...
note: ...which then again requires computing the supertraits of `t1`, completing the cycle.
main.rs:3:1: 4:2 error: unsupported cyclic reference between types/traits detected
main.rs:3 trait t2 : t1 {
main.rs:4 }
note: the cycle begins when computing the supertraits of `t2`...
note: ...which then requires computing the supertraits of `t1`...
note: ...which then again requires computing the supertraits of `t2`, completing the cycle.
error: aborting due to 2 previous errors

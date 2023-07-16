
[01:11:16] ---- num/wrapping.rs - num::wrapping::Wrapping<u128>::is_power_of_two (line 684) stdout ----
[01:11:16] 	error[E0425]: cannot find function `Wrapping` in this scope
[01:11:16]  --> num/wrapping.rs:685:9
[01:11:16]   |
[01:11:16] 4 | assert!(Wrapping(16).is_power_of_two());
[01:11:16]   |         ^^^^^^^^ not found in this scope
[01:11:16] help: possible candidate is found in another module, you can import it into scope
[01:11:16]   |
[01:11:16] 3 | use std::num::Wrapping;
[01:11:16]   |
...

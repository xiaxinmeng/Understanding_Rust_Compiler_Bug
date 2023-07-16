
$ make
rustc m1.rs -C prefer-dynamic
rustc m2.rs -L ./ 2>&1 | grep "error\[E0046\]: not all trait items implemented, missing: .*"
error[E0046]: not all trait items implemented, missing: `CONSTANT`, `Type`, `method`
rustc m2.rs -L ./ 2>&1 | grep "  --> m2.rs:18:1"
  --> m2.rs:18:1
rustc m2.rs -L ./ 2>&1 | grep "   | ^ missing .CONSTANT., .Type., .method. in implementation"
   | ^ missing `CONSTANT`, `Type`, `method` in implementation
rustc m2.rs -L ./ 2>&1 | grep "   = note: .CONSTANT. from trait: .const CONSTANT: u32;."
make: *** [Makefile:8: all] Error 1

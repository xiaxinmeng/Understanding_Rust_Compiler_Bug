shell
% rustc -C opt-level=2 --emit llvm-ir --crate-type=rlib -o opt-level-2.ll test.rust
% rustc -C opt-level=s --emit llvm-ir --crate-type=rlib -o opt-level-s.ll test.rust


% rustc repro.rs --crate-type rlib --edition=2018 -Copt-level=3 -o repro.rlib
% objdump -t repro.rlib |grep test_fn|grep UND
0000000000000000         *UND*	0000000000000000 .hidden _ZN5repro7test_fn17hf3114ef58d23fd06E
0000000000000000         *UND*	0000000000000000 .hidden _ZN5repro7test_fn17hf3114ef58d23fd06E
0000000000000000         *UND*	0000000000000000 .hidden _ZN5repro7test_fn17hf3114ef58d23fd06E

% rustc --version --verbose
rustc 1.56.1 (59eed8a2a 2021-11-01)
binary: rustc
commit-hash: 59eed8a2aac0230a8b53e89d4e99d55912ba6b35
commit-date: 2021-11-01
host: x86_64-unknown-linux-gnu
release: 1.56.1
LLVM version: 13.0.0

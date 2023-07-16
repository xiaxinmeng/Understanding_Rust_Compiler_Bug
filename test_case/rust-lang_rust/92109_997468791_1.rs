
❯ rustc +nightly-2021-09-01 -Zinstrument-mcount -C passes="ee-instrument post-inline-ee-instrument" main.rs --emit asm -Z print-llvm-passes -Z no-parallel-llvm 2>&1 | grep mcount
      Instrument function entry/exit with calls to e.g. mcount() (pre inlining)
      Instrument function entry/exit with calls to e.g. mcount() (post inlining)
❯ rustc +nightly-2021-10-01 -Zinstrument-mcount -C passes="ee-instrument post-inline-ee-instrument" main.rs --emit asm -Z print-llvm-passes -Z no-parallel-llvm 2>&1 | grep mcount
[empty]

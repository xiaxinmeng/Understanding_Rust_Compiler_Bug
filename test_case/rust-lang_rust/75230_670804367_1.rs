sh
$ rustc -C opt-level=0 b.rs && ./b 4
Illegal instruction
$ rustc -C opt-level=1 b.rs && ./b 4
Illegal instruction
$ rustc -C opt-level=2 b.rs && ./b 4
Illegal instruction
$ rustc -C opt-level=3 b.rs && ./b 4
Illegal instruction


$ time rustc +stable main.rs
rustc +stable main.rs  4.94s user 0.18s system 100% cpu 5.080 total
$ time rustc +beta main.rs
rustc +beta main.rs  5.00s user 0.09s system 98% cpu 5.185 total
$ time rustc +nightly main.rs
^C
rustc +nightly main.rs  206.66s user 0.14s system 99% cpu 3:27.45 total
$ time rustc +1aac55d18084910bbfa1d25733a5393860616b8b main.rs
rustc +1aac55d18084910bbfa1d25733a5393860616b8b main.rs  4.06s user 0.05s system 100% cpu 4.092 total

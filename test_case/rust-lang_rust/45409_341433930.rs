
lunch-box. RUST_LOG=debug /usr/bin/time -o killme.master rustc rustc-rust-log.rs 2>&1 | wc -c
606664960
lunch-box. cat killme.master
17.36user 8.18system 0:25.55elapsed 99%CPU (0avgtext+0avgdata 107360maxresident)k
32inputs+8488outputs (4major+20279minor)pagefaults 0swaps

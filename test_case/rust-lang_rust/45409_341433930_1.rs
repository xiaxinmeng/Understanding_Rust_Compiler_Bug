
lunch-box. RUST_LOG=debug /usr/bin/time -o killme rustc rustc-rust-log.rs 2>&1 | wc -c
606664960
lunch-box. cat killme
22.50user 8.48system 0:31.05elapsed 99%CPU (0avgtext+0avgdata 108668maxresident)k
0inputs+8488outputs (0major+20557minor)pagefaults 0swaps

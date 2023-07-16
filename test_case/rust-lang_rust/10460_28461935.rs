
RUST_MIN_STACK=8388608 ./silly-test-spawn-mmap  1.82s user 1.64s system 173% cpu 1.989 total    
RUST_MIN_STACK=8388608 ./silly-test-spawn  2.21s user 10.52s system 207% cpu 6.143 total
RUST_MIN_STACK=8388608 jemalloc.sh ./silly-test-spawn  2.85s user 25.95s system 111% cpu 25.774 total
RUST_MIN_STACK=4194304 jemalloc.sh ./silly-test-spawn  2.88s user 26.56s system 111% cpu 26.367 total                                                                                                               
RUST_MIN_STACK=4194304 ./silly-test-spawn  2.19s user 10.86s system 233% cpu 5.583 total
RUST_MIN_STACK=4194304 ./silly-test-spawn-mmap  2.01s user 10.30s system 116% cpu 10.598 total

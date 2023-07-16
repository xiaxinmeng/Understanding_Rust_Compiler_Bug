 make
# Makefile
all:
    rustc --test foo.rs
    ./foo --shard 1.2 | grep 'test_1 ... ok'
    ./foo --shard 2.2 | grep 'test_2 ... ok'

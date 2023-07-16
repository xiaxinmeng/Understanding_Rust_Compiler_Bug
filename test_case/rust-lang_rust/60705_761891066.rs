
$ RUSTFLAGS= time cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 8.65s
        8.78 real        25.07 user         1.52 sys
$ RUSTFLAGS=-Zsymbol-mangling-version=v0 time cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 8.88s
        9.01 real        26.09 user         1.57 sys

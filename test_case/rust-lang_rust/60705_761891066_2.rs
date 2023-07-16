
$ RUSTFLAGS= time cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 36.67s
       36.80 real       176.98 user        11.27 sys
$ RUSTFLAGS=-Zsymbol-mangling-version=v0 time cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 37.67s
       37.80 real       178.71 user        11.40 sys

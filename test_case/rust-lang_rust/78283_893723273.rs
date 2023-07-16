
C:\Users\steve\Documents\tmp\is-this-ub> cargo +nightly-2021-03-04 rustc --release -- -C codegen-units=1
    Finished release [optimized] target(s) in 0.01s
C:\Users\steve\Documents\tmp\is-this-ub> .\target\release\is-this-ub.exe
thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', src\main.rs:9:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
C:\Users\steve\Documents\tmp\is-this-ub> cargo +nightly-2021-03-05 rustc --release -- -C codegen-units=1
   Compiling is-this-ub v0.1.0 (C:\Users\steve\Documents\tmp\is-this-ub)
    Finished release [optimized] target(s) in 0.68s
C:\Users\steve\Documents\tmp\is-this-ub> .\target\release\is-this-ub.exe
C:\Users\steve\Documents\tmp\is-this-ub> rustc --version
rustc 1.52.1 (9bc8c42bb 2021-05-09)
C:\Users\steve\Documents\tmp\is-this-ub> cargo rustc --release -- -C codegen-units=1
   Compiling is-this-ub v0.1.0 (C:\Users\steve\Documents\tmp\is-this-ub)
    Finished release [optimized] target(s) in 0.94s
C:\Users\steve\Documents\tmp\is-this-ub> .\target\release\is-this-ub.exe
C:\Users\steve\Documents\tmp\is-this-ub>

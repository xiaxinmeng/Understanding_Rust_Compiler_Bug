
xargo/build.sh # do get a libstd with all the MIR, which means the MIR inliner has more options
rustc -Zmir-opt-level=3 --sysroot=$HOME/.xargo/HOST tests/run-pass-fullmir/unsized-tuple-impls.rs

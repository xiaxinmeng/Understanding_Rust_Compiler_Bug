plain
+ python3 ../x.py --stage 2 test --host= --target arm-linux-androideabi
##[group]Building bootstrap
    Finished dev [unoptimized] target(s) in 0.05s
##[endgroup]
thread 'main' panicked at 'Linkcheck currently does not support builds with different hosts and targets.
You can skip linkcheck with --exclude src/tools/linkchecker', test.rs:135:13
Build completed unsuccessfully in 0:00:00

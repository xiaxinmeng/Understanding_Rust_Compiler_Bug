plain
+ python2.7 ../x.py --stage 2 test --exclude src/tools/tidy
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.19s
Skipping Set({test::src/tools/tidy}) because it is excluded
thread 'main' panicked at 'assertion failed: !use_snapshot || stage == 0 || self.local_rebuild', src/bootstrap/builder.rs:1302:9
Build completed unsuccessfully in 0:00:00

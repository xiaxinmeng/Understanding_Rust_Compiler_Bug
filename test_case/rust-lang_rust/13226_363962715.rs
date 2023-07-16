
$ cargo test --target=arm-linux-androideabi
   Compiling android-atomic-13226 v0.1.0 (file:///home/evgeniy/projects/rust-infra/bugs/android-atomic-13226)
    Finished dev [unoptimized + debuginfo] target(s) in 0.88 secs
     Running target/arm-linux-androideabi/debug/deps/android_atomic_13226-a6d2087ae9be837c
Current directory: /home/evgeniy/projects/rust-infra/bugs/android-atomic-13226
/home/evgeniy/projects/rust-infra/bugs/android-atomic-13226/target/arm...26-a6d2087ae9be837c: 1 file pushed. 2.4 MB/s (5396036 bytes in 2.134s

running 2 tests
test uint_nand ... ok
test int_nand ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

ADB_SUCCESS!!!!!!!!!!!!!!
ADB_SUCCESS!!!!!!!!!!!!!!
[evgeniy@15inch android-atomic-13226]$ adb shell
shell@hammerhead:/ $ uname -a
Linux localhost 3.4.0-gcf10b7e #1 SMP PREEMPT Mon Sep 19 22:14:08 UTC 2016 armv7l

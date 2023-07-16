sh
$ rustc -Clinker=./NDK/arm/bin/arm-linux-androideabi-gcc -Car=./NDK/arm/bin/arm-linux-androideabi-ar --test --target=arm-linux-androideabi -g 1.rs
$ adb push ./1 /sdcard/
$ adb shell
(adb)$ cd /sdcard/
(adb)$ RUST_BACKTRACE=full ./1

running 1 test
test test ... FAILED

failures:

---- test stdout ----
	thread 'test' panicked at '0', 1.rs:1
stack backtrace:
   0: 0xb6f3811b - <unknown>
   1: 0xb6f34eaf - <unknown>
   2: 0xb6f3979b - <unknown>
   3: 0xb6f392d7 - <unknown>
   4: 0xb6f39c7b - <unknown>
   5: 0xb6ef71f7 - <unknown>
   6: 0xb6ef73f3 - <unknown>
   7: 0xb6f06e03 - <unknown>
   8: 0xb6f40b4f - <unknown>
   9: 0xb6ef9773 - <unknown>
  10: 0xb6efa59f - <unknown>
  11: 0xb6f40b4f - <unknown>
  12: 0xb6f01623 - <unknown>
  13: 0xb6f38d5b - <unknown>
  14: 0xb6e8c25b - <unknown>
  15: 0xb6e8c3f7 - <unknown>


failures:
    test

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured

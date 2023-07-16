
[jet@~/rust_latest/build] rustc test4.rs
error: linking with `cc` failed with code 1
note: cc arguments: -L/usr/local/lib/rustc/x86_64-unknown-linux-gnu/lib -m64 -o test4 test4.o -L/usr/local/lib/rustc/x86_64-unknown-linux-gnu/lib -lstd-6c65cf4b443341b1-0.7-pre -lrustrt -lrt -lpthread -lrt -ldl -lm -lmorestack -lrustrt -Wl,-rpath,$ORIGIN/../../../../usr/local/lib/rustc/x86_64-unknown-linux-gnu/lib -Wl,-rpath,/usr/local/lib/rustc/x86_64-unknown-linux-gnu/lib
note: test4.o: In function `test1__c_stack_shim':
test4.rc:(.text+0xb2): undefined reference to `test1'
test4.o: In function `test2__c_stack_shim':
test4.rc:(.text+0x16a): undefined reference to `test2'
collect2: error: ld returned 1 exit status

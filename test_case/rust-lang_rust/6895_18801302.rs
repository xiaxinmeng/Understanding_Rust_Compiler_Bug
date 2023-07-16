
                                               ^~~~~~~~~~~~~~
/home/brian/dev/rust2/src/librustc/middle/kind.rs:15:0: 15:21 warning: unused import [-W unused-imports (default)]
/home/brian/dev/rust2/src/librustc/middle/kind.rs:15 use middle::pat_util;
                                                     ^~~~~~~~~~~~~~~~~~~~~
compile_and_link: x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/bin/rustc
error: linking with `cc` failed with code 1
note: cc arguments: -L/opt/devel/rust2/build/x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib -m64 -o x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/bin/rustc x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/bin/rustc.o -L/opt/devel/rust2/build/x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib -lstd-c3ca5d77d81b46c1-0.7-pre -L/opt/devel/rust2/build/x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib -lextra-4782a756585a81-0.7-pre -L/opt/devel/rust2/build/x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib -lsyntax-f25ff37b2c135ff-0.7-pre -L/opt/devel/rust2/build/x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib -lrustc-ba32f36fc1ec12e-0.7-pre -lrustrt -lrt -lpthread -Lrustllvm -lrustllvm -lrt -ldl -lm -lmorestack -lrustrt -Wl,-rpath,$ORIGIN/../lib -Wl,-rpath,/opt/devel/rust2/build/x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib -Wl,-rpath,/usr/local/lib/rustc/x86_64-unknown-linux-gnu/lib
note: /opt/devel/rust2/build/x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib/librustrt.so: undefined reference to `pthread_atfork'
collect2: error: ld returned 1 exit status

error: aborting due to previous error
make: *** [x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/bin/rustc] Error 101

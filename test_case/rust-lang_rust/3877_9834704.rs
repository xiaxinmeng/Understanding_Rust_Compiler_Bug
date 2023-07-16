
compile_and_link: x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib/libsyntax.so
compile_and_link: x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib/librustc.so
cp: x86_64-unknown-linux-gnu/stage1/lib/libsyntax.so
compile_and_link: x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/bin/rustc
cp: x86_64-unknown-linux-gnu/stage1/lib/librustc.so
error: linking with `cc` failed with code 1
note: cc arguments: -L/home/zack/Code/rust/build/x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib -m64 -o x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/bin/rustc x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/bin/rustc.o -L/home/zack/Code/rust/build/x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib -lcore-c3ca5d77d81b46c1-0.4 -L/home/zack/Code/rust/build/x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib -lstd-4782a756585a81-0.4 -L/home/zack/Code/rust/build/x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib -lsyntax-84efebcb12c867a2-0.4 -L/home/zack/Code/rust/build/x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib -lrustc-c84825241471686d-0.4 -lrustrt -lrt -ldl -lm -lmorestack -Wl,-rpath,$ORIGIN/../lib -Wl,-rpath,/home/zack/Code/rust/build/x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib -Wl,-rpath,/usr/local/lib/rustc/x86_64-unknown-linux-gnu/lib
note: /home/zack/Code/rust/build/x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib/libcore-c3ca5d77d81b46c1-0.4.so: undefined reference to `linenoiseHistorySave'
/home/zack/Code/rust/build/x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib/libcore-c3ca5d77d81b46c1-0.4.so: undefined reference to `linenoise'
/home/zack/Code/rust/build/x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib/libcore-c3ca5d77d81b46c1-0.4.so: undefined reference to `linenoiseHistorySetMaxLen'
/home/zack/Code/rust/build/x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib/libcore-c3ca5d77d81b46c1-0.4.so: undefined reference to `linenoiseSetCompletionCallback'
/home/zack/Code/rust/build/x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib/libcore-c3ca5d77d81b46c1-0.4.so: undefined reference to `linenoiseHistoryAdd'
/home/zack/Code/rust/build/x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib/libcore-c3ca5d77d81b46c1-0.4.so: undefined reference to `linenoiseClearScreen'
/home/zack/Code/rust/build/x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib/libcore-c3ca5d77d81b46c1-0.4.so: undefined reference to `linenoiseHistoryLoad'
/home/zack/Code/rust/build/x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib/libcore-c3ca5d77d81b46c1-0.4.so: undefined reference to `linenoiseAddCompletion'

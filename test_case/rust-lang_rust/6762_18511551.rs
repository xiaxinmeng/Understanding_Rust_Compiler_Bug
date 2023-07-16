
lkuper@lenny:~/repos/rust$ x86_64-unknown-linux-gnu/stage2/bin/rustc /home/lkuper/repos/rust/src/test/compile-fail/issue-6762.rs -o x86_64-unknown-linux-gnu/test/compile-fail/issue-6762.stage2-x86_64-unknown-linux-gnu -L x86_64-unknown-linux-gnu/test/compile-fail -L x86_64-unknown-linux-gnu/test/compile-fail/issue-6762.libaux --target=x86_64-unknown-linux-gnu
lkuper@lenny:~/repos/rust$ x86_64-unknown-linux-gnu/test/compile-fail/issue-6762.stage2-x86_64-unknown-linux-gnu 
Segmentation fault (core dumped)
lkuper@lenny:~/repos/rust$ 

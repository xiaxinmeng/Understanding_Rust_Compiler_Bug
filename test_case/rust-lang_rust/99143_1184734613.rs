console
$ cargo clean && cargo +stable build -q && ls -lh target/debug/helloworld.*
-rw-r--r-- 1 lqd 197121  117 Jul 14 19:40 target/debug/helloworld.d
-rwxr-xr-x 2 lqd 197121 156K Jul 14 19:40 target/debug/helloworld.exe*
-rw-r--r-- 2 lqd 197121 1.2M Jul 14 19:40 target/debug/helloworld.pdb

$ cargo clean && cargo +nightly-2022-07-08 build -q && ls -lh target/debug/helloworld.*
-rw-r--r-- 1 lqd 197121  117 Jul 14 19:41 target/debug/helloworld.d
-rwxr-xr-x 2 lqd 197121 157K Jul 14 19:41 target/debug/helloworld.exe*
-rw-r--r-- 2 lqd 197121 1.3M Jul 14 19:41 target/debug/helloworld.pdb

$ cargo clean && cargo +nightly build -q && ls -lh target/debug/helloworld.*
-rw-r--r-- 1 lqd 197121  117 Jul 14 19:41 target/debug/helloworld.d
-rwxr-xr-x 2 lqd 197121 3.4M Jul 14 19:41 target/debug/helloworld.exe*
-rw-r--r-- 2 lqd 197121 2.7M Jul 14 19:41 target/debug/helloworld.pdb

$ cargo clean && cargo +stage1 build -q && ls -lh target/debug/helloworld.*
-rw-r--r-- 1 lqd 197121  117 Jul 14 19:41 target/debug/helloworld.d
-rwxr-xr-x 2 lqd 197121 167K Jul 14 19:41 target/debug/helloworld.exe*
-rw-r--r-- 2 lqd 197121 596K Jul 14 19:41 target/debug/helloworld.pdb

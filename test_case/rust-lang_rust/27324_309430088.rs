
$ uname -a
Linux <REDACTED> 3.10.0-514.10.2.el7.x86_64 #1 SMP Fri Mar 3 00:04:05 UTC 2017 x86_64 x86_64 x86_64 GNU/Linux
$ cargo init --bin hello
     Created binary (application) project
$ cd hello/
$ cargo build
   Compiling hello v0.1.0 (file:///data/ksw/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42 secs
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello`
Hello, world!
$ cargo kcov
kcov: 121 invalid breakpoints skipped in /data/ksw/hello/target/debug/deps/hello-fb494c4f8a209fbc
[..]
$ kcov --verify --debug=4 foo target/debug/hello-fb494c4f8a209fbc
kcov: Address 0x17252 is not at an instruction boundary, skipping
kcov: Address 0x99a8c is not at an instruction boundary, skipping
kcov: Address 0x99b53 is not at an instruction boundary, skipping
kcov: Address 0x99c02 is not at an instruction boundary, skipping
kcov: Address 0x99cac is not at an instruction boundary, skipping
[..lots of others omitted..]

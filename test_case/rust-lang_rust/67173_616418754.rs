
$ cat /etc/centos-release
CentOS release 6.10 (Final)
$ curl https://sh.rustup.rs -sSf | sh
  stable installed -$ cargo build
   Compiling hello-world v0.1.0 (/home/usr15/rust/hello-world)
    Finished dev [unoptimized + debuginfo] target(s) in 1.46s rustc 1.42.0 (b8cedc004 2020-03-09)
$ cargo build --release
   Compiling hello-world v0.1.0 (/home/usr15/rust/hello-world)
    Finished release [optimized] target(s) in 0.32s
$ target/release/hello-world
Hello, world!

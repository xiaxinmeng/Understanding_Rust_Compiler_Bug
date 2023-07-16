
root@d49f02ca0f13:/foo# cargo build --target x86_64-unknown-linux-musl 
   Compiling foo v0.1.0 (/foo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
root@d49f02ca0f13:/foo# target/x86_64-unknown-linux-musl/debug/foo 
Hello, world!
root@d49f02ca0f13:/foo# cargo build --target x86_64-unknown-linux-musl --release
   Compiling foo v0.1.0 (/foo)
    Finished release [optimized] target(s) in 0.17s

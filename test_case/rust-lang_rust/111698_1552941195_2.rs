
$ RUSTFLAGS="-Ctarget-feature=+crt-static" cargo build --release
   Compiling x v0.1.0 (/tmp/mytemp.iKnNJK/x)
    Finished release [optimized] target(s) in 2.96s
$ file target/release/x
target/release/x: ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), static-pie linked, BuildID[sha1]=f1dc0a1ca6ee75bb3b96ab24bdeacf82379f1305, with debug_info, not stripped
$ ./target/release/x       
Hello, world!

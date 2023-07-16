
$ RUSTFLAGS="-Ctarget-feature=+crt-static" cargo build --release 
   Compiling libc v0.2.144
   Compiling memmap2 v0.5.10
   Compiling xkbcommon v0.5.0
   Compiling x v0.1.0 (/tmp/mytemp.iKnNJK/x)
    Finished release [optimized] target(s) in 3.53s
$ file target/release/x                                                           
target/release/x: ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), dynamically linked, BuildID[sha1]=221da58c22e8613b8972fe45c140e1c66396699c, with debug_info, not stripped
$ ./target/release/x       
zsh: segmentation fault (core dumped)  ./target/release/x

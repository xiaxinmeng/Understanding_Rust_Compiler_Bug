sh
$ rustc -C rpath 1.rs
$ otool -l ./1
...
Load command 13
          cmd LC_RPATH
      cmdsize 120
         path @loader_path/../../../.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib (offset 12)
Load command 14
          cmd LC_RPATH
      cmdsize 64
         path /usr/local/lib/rustlib/x86_64-apple-darwin/lib (offset 12)
...

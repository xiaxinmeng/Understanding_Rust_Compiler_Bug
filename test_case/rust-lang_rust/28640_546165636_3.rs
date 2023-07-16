
% cd /tmp/foobar
% rustc -C prefer-dynamic -C rpath /tmp/a.rs
% otool -lvv a | grep -A 2 RPATH       
          cmd LC_RPATH
      cmdsize 136
         path @loader_path/../../../Users/comex/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib (offset 12)
--
          cmd LC_RPATH
      cmdsize 72
         path /private/tmp/foobar/lib/rustlib/x86_64-apple-darwin/lib (offset 12)

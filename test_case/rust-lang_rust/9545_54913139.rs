
[m@rhel rust-nightly-i686-unknown-linux-gnu]$ LD_LIBRARY_PATH=lib bin/rustc
bin/rustc: /usr/lib/libstdc++.so.6: version `GLIBCXX_3.4.15' not found (required by lib/librustc_llvm-4e7c5e5c.so)
[m@rhel rust-nightly-i686-unknown-linux-gnu]$ cat /etc/redhat-release 
CentOS release 6.5 (Final)

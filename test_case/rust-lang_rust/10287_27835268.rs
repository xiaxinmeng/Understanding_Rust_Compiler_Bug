
-> % adb shell
root@generic:/ # cd /data/tmp/rust
root@generic:/data/tmp/rust # ls
hello
libgnustl_shared.so
librustrt.so
librustuv-d4277cd5f62aa99-0.9-pre.so
libstd-6c65cf4b443341b1-0.9-pre.so
root@generic:/data/tmp/rust # LD_LIBRARY_PATH=. ./hello
Hello World!

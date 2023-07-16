shell session
$ while command sleep 5; do cargo clean; if cargo check --release |& grep SIGSEGV; then echo segfault; else echo no segfault; fi; done
segfault
no segfault
segfault
segfault
segfault
segfault
segfault
segfault
segfault
segfault
segfault
no segfault

shell
$ ./x.py dist rustc
(cut)
$ tar wtf build/dist/rustc-1.64.0-dev-x86_64-unknown-linux-gnu.tar.gz | sed -n 's/rustc-[^/]*\/rustc//p' | grep proc-macro-srv
/libexec/rust-analyzer-proc-macro-srv

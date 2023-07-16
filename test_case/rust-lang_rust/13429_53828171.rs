
-> % rustc bad-float-cast.rs --target=i686-apple-darwin -C target-feature="+sse2"
-> % ./bad-float-cast
3.14

-> % rustc bad-float-cast.rs --target=i686-apple-darwin
-> % ./bad-float-cast
126443840048

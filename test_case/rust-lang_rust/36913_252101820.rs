
$ rustc +nightly -V
rustc 1.14.0-nightly (3210fd5c2 2016-10-05)
$ rustc +nightly --print cfg --target armv7-unknown-linux-gnueabihf | grep feature
target_feature="vfp2"
target_feature="vfp3"

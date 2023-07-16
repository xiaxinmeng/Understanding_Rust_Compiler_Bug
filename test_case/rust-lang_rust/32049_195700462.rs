
$ multirust default nightly-2016-03-08
$ multirust add-target nightly-2016-03-08 arm-unknown-linux-gnueabihf
$ multirust add-target nightly-2016-03-08 i686-unknown-linux-gnu
$ rustc --target arm-unknown-linux-gnueabihf 32049.rs --crate-type lib -O -g
# bug
$ rustc --target i686-unknown-linux-gnu 32049.rs --crate-type lib -O -g
# no bug

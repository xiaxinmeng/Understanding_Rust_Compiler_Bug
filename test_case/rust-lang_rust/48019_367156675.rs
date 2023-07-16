
$ rustc +nightly -L dependency=/bar1 /dev/null --crate-type rlib && sha1sum libnull.rlib
b7aae8f50e259300dd62275d8d1de55e1bc13cdb  libnull.rlib
$ rustc +nightly -L dependency=/bar2 /dev/null --crate-type rlib && sha1sum libnull.rlib     
f91e3c3a28aef26cc06ad789339822b22e0f5121  libnull.rlib

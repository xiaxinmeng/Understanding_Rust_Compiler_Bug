
$ rm -rf target
$ cargo +nightly build -p serde_derive && md5 target/debug/deps/libserde_derive*.dylib
MD5 (target/debug/deps/libserde_derive-e81c055c9829c0e5.dylib) = 81edae9023e3aba77074074ca415a013
$ rm -rf target/debug/deps/libserde_derive* && cargo +nightly build -p serde_derive && md5 target/debug/deps/libserde_derive*.dylib
MD5 (target/debug/deps/libserde_derive-e81c055c9829c0e5.dylib) = ed0b3fc7813d7ec6c7ffcb44fb290550
$ rm -rf target/debug/deps/libserde_derive* && cargo +nightly build -p serde_derive && md5 target/debug/deps/libserde_derive*.dylib
MD5 (target/debug/deps/libserde_derive-e81c055c9829c0e5.dylib) = ed0b3fc7813d7ec6c7ffcb44fb290550
# ...

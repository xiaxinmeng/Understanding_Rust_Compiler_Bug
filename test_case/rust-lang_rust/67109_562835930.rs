console
> find -name '*libLLVM*' -exec ls -lai {} \+
129111035 -rw-r--r-- 1 lzutao lzutao 81161488 Dec  3 09:33 ./beta-x86_64-unknown-linux-gnu/lib/libLLVM-9-rust-1.40.0-beta.so
129111071 -rw-r--r-- 1 lzutao lzutao 81161488 Dec  3 09:33 ./beta-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libLLVM-9-rust-1.40.0-beta.so
126485192 -rw-r--r-- 1 lzutao lzutao 83195704 Dec  5 12:54 ./master/lib/libLLVM-9-rust-1.41.0-nightly.so
126485193 -rw-r--r-- 1 lzutao lzutao 83195704 Dec  5 12:54 ./master/lib/rustlib/x86_64-unknown-linux-gnu/lib/libLLVM-9-rust-1.41.0-nightly.so
126755208 -rw-r--r-- 1 lzutao lzutao 83195704 Dec  7 04:09 ./nightly-x86_64-unknown-linux-gnu/lib/libLLVM-9-rust-1.41.0-nightly.so
126755209 -rw-r--r-- 1 lzutao lzutao 83195704 Dec  7 04:09 ./nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libLLVM-9-rust-1.41.0-nightly.so
> find -name '*libLLVM*' -exec sha256sum {} \+
b837248c30682a65177aa00ea138f33d7ad2596fd7cf158d41cb5d858ebe2df7  ./nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libLLVM-9-rust-1.41.0-nightly.so
b837248c30682a65177aa00ea138f33d7ad2596fd7cf158d41cb5d858ebe2df7  ./nightly-x86_64-unknown-linux-gnu/lib/libLLVM-9-rust-1.41.0-nightly.so
4fdc606fff426a248fc6b814aabd2ff9087f13893b9bfb6ed0724ad89c39995a  ./beta-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libLLVM-9-rust-1.40.0-beta.so
4fdc606fff426a248fc6b814aabd2ff9087f13893b9bfb6ed0724ad89c39995a  ./beta-x86_64-unknown-linux-gnu/lib/libLLVM-9-rust-1.40.0-beta.so
b837248c30682a65177aa00ea138f33d7ad2596fd7cf158d41cb5d858ebe2df7  ./master/lib/rustlib/x86_64-unknown-linux-gnu/lib/libLLVM-9-rust-1.41.0-nightly.so
b837248c30682a65177aa00ea138f33d7ad2596fd7cf158d41cb5d858ebe2df7  ./master/lib/libLLVM-9-rust-1.41.0-nightly.so

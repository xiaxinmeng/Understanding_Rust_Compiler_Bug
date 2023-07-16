bash
$ cargo new mcve
$ cd mcve
$ mkdir .cargo
$ rustup install nightly-2021-05-22
$ rustup target add thumbv6m-none-eabi --toolchain nightly-2021-05-22
$ rustup override set nightly-2021-05-22

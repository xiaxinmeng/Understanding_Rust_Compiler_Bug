
% docker run -ti --rm ubuntu:16.04
$ apt update && apt install curl gcc git
$ curl https://sh.rustup.rs -sSf | sh
$ . ~/.cargo/env
$ rustup toolchain install nightly-2017-07-28
$ rustup toolchain install nightly-2017-08-01
$ git clone https://gitlab.com/remram44/rs-web --single-branch --branch ICE
$ cd rs-web
$ cargo +nightly-2017-07-28 build
$ # Observe ICE
$ cargo +nightly-2017-08-01 build
$ # Observe no ICE, build time 8 minutes
$ git checkout 8fca3d957f91115349aaeb3ab58056b92d13b953
$ # Now on previous version, that always worked
$ cargo +nightly-2017-07-28 build
$ # Observe build time 6 seconds
$ cargo +nightly-2017-08-01 build
$ # Observe build time 1 minute 51 seconds

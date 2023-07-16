
find ~/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/ -name \*.so | xargs -t -n 1 /usr/bin/dwarfdump -i > /dev/null

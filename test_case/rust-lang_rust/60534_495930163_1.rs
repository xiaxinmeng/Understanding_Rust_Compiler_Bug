bash
instdir="${HOME}/build/2nonpkgs/rust.stuff/rust/rust.installed.dir"
time python2 ./x.py build $verbose -j $threads
time python2 ./x.py install $verbose -j $threads
rustup toolchain link master-installed "${instdir}"
rustup toolchain link master-stage1 build/x86_64-unknown-linux-gnu/stage1
rustup toolchain link master-stage2 build/x86_64-unknown-linux-gnu/stage2
rustup default master-installed

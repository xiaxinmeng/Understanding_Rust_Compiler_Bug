sh
mkdir issue-88769
cd issue-88769

curl -sSO https://gist.githubusercontent.com/eddyb/08ea4736082d9b2326a984c4cb828a08/raw/6a2b96cd8422950fa2b160e518671df66965a863/issue-88769-min.rs

rustup toolchain install nightly-2021-08-22 -c llvm-tools-preview

# This will crash - ignore the crash and keep going, `-C save-temps` works.
rustc +nightly-2021-08-22 issue-88769-min.rs -O -C save-temps

# This reproduces the crash using the `.bc` from `-C save-temps`.
~/.rustup/toolchains/nightly-2021-08-22-x*64*/lib/rustlib/x*64*/bin/opt -O3 issue-88769-min.*-cgu.3.rcgu.no-opt.bc > opt.bc

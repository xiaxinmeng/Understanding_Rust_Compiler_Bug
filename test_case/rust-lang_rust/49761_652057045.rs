
$ time rustc +1.25.0 -vV
rustc 1.25.0 (84203cac6 2018-03-25)
binary: rustc
commit-hash: 84203cac67e65ca8640b8392348411098c856985
commit-date: 2018-03-25
host: x86_64-unknown-linux-gnu
release: 1.25.0
LLVM version: 6.0

real    0m0,096s
user    0m0,073s
sys     0m0,004s
$ time rustc +1.25.0 -vV
rustc 1.25.0 (84203cac6 2018-03-25)
binary: rustc
commit-hash: 84203cac67e65ca8640b8392348411098c856985
commit-date: 2018-03-25
host: x86_64-unknown-linux-gnu
release: 1.25.0
LLVM version: 6.0

real    0m0,074s
user    0m0,074s
sys     0m0,000s
$ time rustc +1.35.0 -vV
rustc 1.35.0 (3c235d560 2019-05-20)
binary: rustc
commit-hash: 3c235d5600393dfe6c36eeed34042efad8d4f26e
commit-date: 2019-05-20
host: x86_64-unknown-linux-gnu
release: 1.35.0
LLVM version: 8.0

real    0m0,033s
user    0m0,033s
sys     0m0,000s
$ time rustc +1.35.0 -vV
rustc 1.35.0 (3c235d560 2019-05-20)
binary: rustc
commit-hash: 3c235d5600393dfe6c36eeed34042efad8d4f26e
commit-date: 2019-05-20
host: x86_64-unknown-linux-gnu
release: 1.35.0
LLVM version: 8.0

real    0m0,046s
user    0m0,028s
sys     0m0,000s
$ time rustc -vV
rustc 1.44.1 (c7087fe00 2020-06-17)
binary: rustc
commit-hash: c7087fe00d2ba919df1d813c040a5d47e43b0fe7
commit-date: 2020-06-17
host: x86_64-unknown-linux-gnu
release: 1.44.1
LLVM version: 9.0

real    0m0,034s
user    0m0,006s
sys     0m0,012s
$ time rustc -vV
rustc 1.44.1 (c7087fe00 2020-06-17)
binary: rustc
commit-hash: c7087fe00d2ba919df1d813c040a5d47e43b0fe7
commit-date: 2020-06-17
host: x86_64-unknown-linux-gnu
release: 1.44.1
LLVM version: 9.0

real    0m0,018s
user    0m0,005s
sys     0m0,013s

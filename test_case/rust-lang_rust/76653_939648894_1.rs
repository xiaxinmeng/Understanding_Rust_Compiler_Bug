
$ du -d2 -h .git/modules/ | sort -h
472K	.git/modules/src/rust-installer
1.5M	.git/modules/library/backtrace
13M	.git/modules/library/stdarch
14M	.git/modules/library
52M	.git/modules/src/tools
53M	.git/modules/src
67M	.git/modules/
$ du -d1 -h .git/modules/src/tools/ | sort -h
8.7M	.git/modules/src/tools/miri
9.5M	.git/modules/src/tools/rls
34M	.git/modules/src/tools/cargo

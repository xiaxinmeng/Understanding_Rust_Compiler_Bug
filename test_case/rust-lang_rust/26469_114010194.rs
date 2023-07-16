
~/Source/rust/rust-master/BUILD (~/S/r/r: master)
tpg@ted$ head config.mk

CFG_ENABLE_LLVM_ASSERTIONS := 1
CFG_LOCALSTATEDIR    := /var/lib
CFG_SYSCONFDIR       := /etc
CFG_DATADIR          := /share
CFG_INFODIR          := /share/info
CFG_LLVM_ROOT        :=
CFG_PYTHON           :=
CFG_JEMALLOC_ROOT    :=
CFG_BUILD            := x86_64-unknown-linux-gnu
~/Source/rust/rust-master/BUILD (~/S/r/r: master)
tpg@ted$ LD_LIBRARY_PATH=/home/tpg/apps/rustc-debug/lib /home/tpg/apps/rustc-debug/bin/rustc  --version --verbose
rustc 1.2.0-dev (31d9aee68 2015-06-22)
binary: rustc
commit-hash: 31d9aee68499cba8d1a5332b841eca5c67991001
commit-date: 2015-06-22
host: x86_64-unknown-linux-gnu
release: 1.2.0-dev
~/Source/rust/rust-master/BUILD (~/S/r/r: master)
tpg@ted$

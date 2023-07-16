
$ rustup show
Default host: x86_64-unknown-linux-gnu

installed toolchains
--------------------

nightly-x86_64-unknown-linux-gnu
master-installed (default)
master-stage1
master-stage2

active toolchain
----------------

master-installed (default)
rustc 1.36.0-dev (315ab95a9 2019-05-25)

$ rls
error: 'rls' is not installed for the toolchain 'master-installed'
To install, run `rustup component add rls`

$ rustup component add rls
error: toolchain 'master-installed' does not support components
info: caused by: invalid toolchain name: 'master-installed'


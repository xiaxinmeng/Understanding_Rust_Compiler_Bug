
% git log -1
commit 05812fa8c588473f03e3fa7cf59cd84f4f37c715 (HEAD -> moz-master, moz-gh/master)
Merge: 0db7abe5b6 3c25f80f85
Author: bors <bors@rust-lang.org>
Date:   Wed Oct 31 06:42:24 2018 +0000

    Auto merge of #55304 - alexcrichton:update-credentials, r=kennytm

    ci: Move global credentials to web configuration

    This commit moves a number of our encrypted credentials stored in
    configuration files in this repository to env vars on the web UI. This
    will hopefully make it easier to rotate credentials in the future as
    well as quickly change them if the need arises. (quicker than landing a
    PR that is).

    This also updates the travis deployment process to always use the `aws`
    command line tool which we're already installing on Linux and should
    enable us to avoid all `dpl` gem issues as well as have greater control
    over what's going where.
% ./build/x86_64-unknown-linux-gnu/stage1/bin/rustc --version -v
rustc 1.31.0-dev
binary: rustc
commit-hash: unknown
commit-date: unknown
host: x86_64-unknown-linux-gnu
release: 1.31.0-dev
LLVM version: 8.0


testing https://github.com/servo/servo
hint: Using 'master' as the name for the initial branch. This default branch name
hint: is subject to change. To configure the initial branch name to use in all
hint: of your new repositories, which will suppress this warning, call:
hint: 
hint: 	git config --global init.defaultBranch <name>
hint: 
hint: Names commonly chosen instead of 'master' are 'main', 'trunk' and
hint: 'development'. The just-created branch can be renamed via this command:
hint: 
hint: 	git branch -m <name>
Initialized empty Git repository in /checkout/obj/build/ct/servo/.git/
fatal: Could not parse object '785a344e32db58d4e631fd3cae17fd1f29a721ab'.
From https://github.com/servo/servo
 * branch                master     -> FETCH_HEAD
fatal: Could not parse object '785a344e32db58d4e631fd3cae17fd1f29a721ab'.
From https://github.com/servo/servo
 * branch                master     -> FETCH_HEAD
fatal: Could not parse object '785a344e32db58d4e631fd3cae17fd1f29a721ab'.
From https://github.com/servo/servo
 * branch                  master     -> FETCH_HEAD
fatal: Could not parse object '785a344e32db58d4e631fd3cae17fd1f29a721ab'.
error: RPC failed; curl 56 GnuTLS recv error (-54): Error in the pull function.
error: 5870 bytes of body are still expected
fetch-pack: unexpected disconnect while reading sideband packet
fatal: early EOF
fatal: fetch-pack: invalid index-pack output
thread 'main' panicked at 'assertion failed: status.success()', src/tools/cargotest/main.rs:148:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Build completed unsuccessfully in 0:15:54
make: *** [Makefile:44: check-aux] Error 1

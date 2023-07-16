plain
Updating files:  99% (35444/35802)
Updating files:  99% (35711/35802)
Updating files: 100% (35802/35802)
Updating files: 100% (35802/35802), done.
Note: switching to 'refs/remotes/pull/100101/merge'.
You are in 'detached HEAD' state. You can look around, make experimental
changes and commit them, and you can discard any commits you make in this
state without impacting any branches by switching back to a branch.

---
  git switch -

Turn off this advice by setting config variable advice.detachedHead to false

HEAD is now at d1ab5273 Merge e8939840ed9a43d2c7254c1473b4975106a8b7b6 into bb99e6fdd99b0a9a9f75bc60b0995b4ef8e752ab
[command]"C:\Program Files\Git\bin\git.exe" log -1 --format='%H'
'd1ab527340759fa47521f6dac9e7558f96433510'
'd1ab527340759fa47521f6dac9e7558f96433510'
##[group]Run echo "[CI_PR_NUMBER=$num]"
echo "[CI_PR_NUMBER=$num]"
env:
  CI_JOB_NAME: x86_64-msvc-1
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
---
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.11s
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `2`: x.py should be run with `--stage 2` on CI, but was run with `--stage 1`', config.rs:1287:21
Build completed unsuccessfully in 0:00:00
Build completed unsuccessfully in 0:00:00
make: *** [Makefile:73: ci-subset-1] Error 1

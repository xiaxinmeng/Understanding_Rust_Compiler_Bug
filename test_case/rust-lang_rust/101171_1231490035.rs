plain
Updating files:  98% (35222/35940)
Updating files:  99% (35581/35940)
Updating files: 100% (35940/35940)
Updating files: 100% (35940/35940), done.
Note: switching to 'refs/remotes/pull/101171/merge'.
You are in 'detached HEAD' state. You can look around, make experimental
changes and commit them, and you can discard any commits you make in this
state without impacting any branches by switching back to a branch.

---
HEAD is now at f669aebf Merge 9d4e79b9a4b54b5b56dfe0fa48a0007a618169ce into a0d07093f80a0206f42d3dbada66212eda52b694
##[endgroup]
[command]"C:\Program Files\Git\bin\git.exe" log -1 --format='%H'
'f669aebf3083ede6432587ace510d0ac3995a1ed'
##[group]Run echo "[CI_PR_NUMBER=$num]"
echo "[CI_PR_NUMBER=$num]"
env:
  CI_JOB_NAME: x86_64-msvc-1
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
---
   Compiling object v0.26.2
   Compiling hashbrown v0.12.3
   Compiling std_detect v0.1.5 (D:\a\rust\rust\library\stdarch\crates\std_detect)
   Compiling addr2line v0.16.0
error: unused variable: `header_sz`
    |
    |
708 |             let header_sz = mem::size_of::<c::FILE_ID_BOTH_DIR_INFO>();
    |                 ^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_header_sz`
    |
    = note: `-D unused-variables` implied by `-D warnings`
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:01:13
Build completed unsuccessfully in 0:01:13
make: *** [Makefile:73: ci-subset-1] Error 1

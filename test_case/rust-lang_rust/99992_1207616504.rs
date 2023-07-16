plain
Updating files:  98% (34930/35642)
Updating files:  99% (35286/35642)
Updating files: 100% (35642/35642)
Updating files: 100% (35642/35642), done.
Note: switching to 'refs/remotes/pull/99992/merge'.
You are in 'detached HEAD' state. You can look around, make experimental
changes and commit them, and you can discard any commits you make in this
state without impacting any branches by switching back to a branch.

---
  git switch -

Turn off this advice by setting config variable advice.detachedHead to false

HEAD is now at a4ea818d Merge 0b5b57c6674c90f6151506293eb7706705b35b78 into 93ab13b4e894ab74258c40aaf29872db2b17b6b4
[command]"C:\Program Files\Git\bin\git.exe" log -1 --format='%H'
'a4ea818df2dcfb2e076a028552c7a652efb6eb28'
'a4ea818df2dcfb2e076a028552c7a652efb6eb28'
##[group]Run echo "[CI_PR_NUMBER=$num]"
echo "[CI_PR_NUMBER=$num]"
env:
  CI_JOB_NAME: i686-mingw-1
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
---
LowFree:        25406744 kB
SwapTotal:       4718592 kB
SwapFree:        4718592 kB
+ make ci-mingw-subset-1
D:\a\rust\rust//x.sh: line 27: exec: py -3: not found
make: *** [Makefile:83: ci-mingw-subset-1] Error 127

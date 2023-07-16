plain
Resolving deltas:  99% (3279/3312)
Resolving deltas: 100% (3312/3312)
Resolving deltas: 100% (3312/3312), done.
From https://github.com/rust-lang/rust
 * [new ref]           d991608a130b8732115226024472c1796f9e1db2 -> pull/88797/merge
##[group]Determining the checkout info
##[endgroup]
##[group]Checking out the ref
[command]"C:\Program Files\Git\bin\git.exe" checkout --progress --force refs/remotes/pull/88797/merge
---
Updating files:  98% (29643/30247)
Updating files:  99% (29945/30247)
Updating files: 100% (30247/30247)
Updating files: 100% (30247/30247), done.
Note: switching to 'refs/remotes/pull/88797/merge'.
You are in 'detached HEAD' state. You can look around, make experimental
changes and commit them, and you can discard any commits you make in this
state without impacting any branches by switching back to a branch.

---
  git switch -

Turn off this advice by setting config variable advice.detachedHead to false

HEAD is now at d991608a Merge c52a11d94507fa20b633f9f56ac68c176e115b65 into 497ee321af3b8496eaccd7af7b437f18bab81abf
[command]"C:\Program Files\Git\bin\git.exe" log -1 --format='%H'
'd991608a130b8732115226024472c1796f9e1db2'
'd991608a130b8732115226024472c1796f9e1db2'
##[group]Run echo "[CI_PR_NUMBER=$num]"
echo "[CI_PR_NUMBER=$num]"
env:
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
---

Files: 381
Size:       1919168559
Compressed: 191037739
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
env: =
##[error]Unable to process file command 'env' successfully.
##[error]Value cannot be null. (Parameter 'name')
##[error]The parameter is incorrect.
Cleaning up orphan processes
Terminate orphan process: pid (6128) (node)
Terminate orphan process: pid (3132) (python)

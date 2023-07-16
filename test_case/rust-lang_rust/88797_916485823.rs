plain
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

HEAD is now at cce7c1a5 Merge 8e8a40d23446181fe755c149286265b96941a66f into 497ee321af3b8496eaccd7af7b437f18bab81abf
[command]"C:\Program Files\Git\bin\git.exe" log -1 --format='%H'
'cce7c1a542f3b660dde258d97899667845b9c441'
'cce7c1a542f3b660dde258d97899667845b9c441'
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
src/ci/scripts/install-clang.sh: line 58: cd: src/ci/scripts: No such file or directory
C:/Program : The term 'C:/Program' is not recognized as the name of a cmdlet, function, script file, or operable 
program. Check the spelling of the name, or if a path was included, verify that the path is correct and try again.
At line:1 char:1
+ C:/Program Files/Git/windows-select-sdk.ps1
+ ~~~~~~~~~~
    + CategoryInfo          : ObjectNotFound: (C:/Program:String) [], CommandNotFoundException
    + FullyQualifiedErrorId : CommandNotFoundException
##[error]Process completed with exit code 1.
Post job cleanup.
[command]"C:\Program Files\Git\bin\git.exe" version
git version 2.33.0.windows.2

plain
Resolving deltas:  99% (3283/3316)
Resolving deltas: 100% (3316/3316)
Resolving deltas: 100% (3316/3316), done.
From https://github.com/rust-lang/rust
 * [new ref]           d29573c65908158a38c5e5804d75d3af2d6dbd4a -> pull/88797/merge
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

HEAD is now at d29573c6 Merge 65df64b00a6d1e79e49ba080b290edef0df1e8bd into 497ee321af3b8496eaccd7af7b437f18bab81abf
[command]"C:\Program Files\Git\bin\git.exe" log -1 --format='%H'
'd29573c65908158a38c5e5804d75d3af2d6dbd4a'
'd29573c65908158a38c5e5804d75d3af2d6dbd4a'
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
WARNING: User declined to install module (VCVars).
Invoke-VCVars : The term 'Invoke-VCVars' is not recognized as the name of a cmdlet, function, script file, or operable 
program. Check the spelling of the name, or if a path was included, verify that the path is correct and try again.
At D:\a\rust\rust\src\ci\scripts\windows-select-sdk.ps1:2 char:1
+ Invoke-VCVars -t ARM64 -s 19041
+ ~~~~~~~~~~~~~
    + CategoryInfo          : ObjectNotFound: (Invoke-VCVars:String) [], CommandNotFoundException
    + FullyQualifiedErrorId : CommandNotFoundException
 
##[error]Unable to process file command 'env' successfully.
##[error]Invalid environment variable format ''
##[error]The parameter is incorrect.
Cleaning up orphan processes
Terminate orphan process: pid (2544) (node)
Terminate orphan process: pid (6700) (python)

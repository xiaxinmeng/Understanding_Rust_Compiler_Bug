plain
Resolving deltas:  99% (3281/3314)
Resolving deltas: 100% (3314/3314)
Resolving deltas: 100% (3314/3314), done.
From https://github.com/rust-lang/rust
 * [new ref]           aa57ac579165b0784b4688ca31440165ab48e0e9 -> pull/88797/merge
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

HEAD is now at aa57ac57 Merge da8665e428a49c93d043f4b0783499230d21a685 into 497ee321af3b8496eaccd7af7b437f18bab81abf
[command]"C:\Program Files\Git\bin\git.exe" log -1 --format='%H'
'aa57ac579165b0784b4688ca31440165ab48e0e9'
'aa57ac579165b0784b4688ca31440165ab48e0e9'
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
Invoke-VCVars : Cannot process argument transformation on parameter 'SDK'. Cannot convert value "19041" to type 
"System.Version". Error: "Version string portion was too short or too long."
At D:\a\rust\rust\src\ci\scripts\windows-select-sdk.ps1:2 char:27
+ Invoke-VCVars -t ARM64 -s 19041
+                           ~~~~~
    + CategoryInfo          : InvalidData: (:) [Invoke-VCVars], ParameterBindingArgumentTransformationException
    + FullyQualifiedErrorId : ParameterArgumentTransformationError,Invoke-VCVars
 
##[error]Unable to process file command 'env' successfully.
##[error]Invalid environment variable format ''
##[error]The parameter is incorrect.
Cleaning up orphan processes
Terminate orphan process: pid (1508) (node)
Terminate orphan process: pid (2616) (python)

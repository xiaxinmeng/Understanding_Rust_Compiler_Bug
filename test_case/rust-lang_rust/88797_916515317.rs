plain
Updating files:  99% (29945/30247)
Updating files:  99% (29993/30247)
Updating files: 100% (30247/30247)
Updating files: 100% (30247/30247), done.
Note: switching to 'refs/remotes/pull/88797/merge'.
You are in 'detached HEAD' state. You can look around, make experimental
changes and commit them, and you can discard any commits you make in this
state without impacting any branches by switching back to a branch.

---
  git switch -

Turn off this advice by setting config variable advice.detachedHead to false

HEAD is now at 0b9a7ce8 Merge 88ebd528d98fcd4d22c84108034d2cbb0a932f05 into 497ee321af3b8496eaccd7af7b437f18bab81abf
[command]"C:\Program Files\Git\bin\git.exe" log -1 --format='%H'
'0b9a7ce821804dfe4699117397a96c40848b2620'
'0b9a7ce821804dfe4699117397a96c40848b2620'
##[group]Run echo "[CI_PR_NUMBER=$num]"
echo "[CI_PR_NUMBER=$num]"
env:
  CI_JOB_NAME: dist-aarch64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
---
  EXTRA_VARIABLES: {
 "RUST_CONFIGURE_ARGS": "--build=x86_64-pc-windows-msvc --host=aarch64-pc-windows-msvc --enable-full-tools --enable-profiler",
 "SCRIPT": "python x.py dist",
 "DIST_REQUIRE_ALL_TOOLS": 0,
 "SELECT_WINDOWS_SDK": 1
##[endgroup]
adding extra environment variable DIST_REQUIRE_ALL_TOOLS
adding extra environment variable RUST_CONFIGURE_ARGS
adding extra environment variable SCRIPT
---

Files: 381
Size:       1919168559
Compressed: 191037739
ParserError: D:\a\rust\rust\src\ci\scripts\windows-select-sdk.ps1:12
Line |
  12 |  | ForEach-Object { Add-Content -Path ${env:GITHUB_PATH}" -Value $_ }
     | The string is missing the terminator: ".

##[error]Process completed with exit code 1.
Post job cleanup.

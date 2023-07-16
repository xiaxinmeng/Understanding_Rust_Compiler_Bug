plain
Updating files:  98% (34932/35644)
Updating files:  99% (35288/35644)
Updating files: 100% (35644/35644)
Updating files: 100% (35644/35644), done.
Note: switching to 'refs/remotes/pull/100292/merge'.
You are in 'detached HEAD' state. You can look around, make experimental
changes and commit them, and you can discard any commits you make in this
state without impacting any branches by switching back to a branch.

---
  git switch -

Turn off this advice by setting config variable advice.detachedHead to false

HEAD is now at 06787b0c Merge 374bc90c579230784c006d4813666cd6e80a46f7 into f03ce30962cf1b2a5158667eabae8bf6e8d1cb03
[command]"C:\Program Files\Git\bin\git.exe" log -1 --format='%H'
'06787b0c88be30db50b1be69aca0ac4b5fdb5c94'
'06787b0c88be30db50b1be69aca0ac4b5fdb5c94'
##[group]Run echo "[CI_PR_NUMBER=$num]"
echo "[CI_PR_NUMBER=$num]"
env:
  CI_JOB_NAME: x86_64-msvc-exfat
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
---
running 1 test
F
failures:

---- sys::windows::fs::tests::test_exfat_please stdout ----
thread 'sys::windows::fs::tests::test_exfat_please' panicked at 'couldn't mount exfat drive
"\u{1b}[91mmountexfat.ps1: \u{1b}[91mThe term 'New-VHD' is not recognized as a name of a cmdlet, function, script file, or executable program.\r\nCheck the spelling of the name, or if a path was included, verify that the path is correct and try again.\u{1b}[0m\r\n"', library\std\src\sys\windows\fs\tests.rs:28:17


failures:
    sys::windows::fs::tests::test_exfat_please
    sys::windows::fs::tests::test_exfat_please

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 908 filtered out; finished in 35.71s

error: test failed, to rerun pass '-p std --lib'
Build completed unsuccessfully in 0:04:47

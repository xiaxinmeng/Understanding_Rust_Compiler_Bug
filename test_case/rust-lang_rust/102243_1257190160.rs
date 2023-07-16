plain
Updating files:  98% (37295/38056)
Updating files:  99% (37676/38056)
Updating files: 100% (38056/38056)
Updating files: 100% (38056/38056), done.
Note: switching to 'refs/remotes/pull/102243/merge'.
You are in 'detached HEAD' state. You can look around, make experimental
changes and commit them, and you can discard any commits you make in this
state without impacting any branches by switching back to a branch.

---
HEAD is now at bae2e926 Merge 618b0546ec6cc5395d4819374110db672aae52f5 into 6580010551063718462f9dfe41c9490d92994d0e
##[endgroup]
[command]"C:\Program Files\Git\bin\git.exe" log -1 --format='%H'
'bae2e926c18de2e5195281bb538ace567fbed892'
##[group]Run echo "[CI_PR_NUMBER=$num]"
echo "[CI_PR_NUMBER=$num]"
env:
  CI_JOB_NAME: i686-mingw-2
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
---
   Compiling unic-langid-macros-impl v0.9.0
error: could not compile `unic-langid-macros-impl`

Caused by:
  process didn't exit successfully: `D:\a\rust\rust\build\bootstrap\debug\rustc --crate-name unic_langid_macros_impl --edition=2018 C:\Users\runneradmin\.cargo\registry\src\github.com-1285ae84e5963aae\unic-langid-macros-impl-0.9.0\src\lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type proc-macro --emit=dep-info,link -C prefer-dynamic -C embed-bitcode=no -C debuginfo=0 -Zunstable-options --check-cfg values(feature) --check-cfg names() --check-cfg values() -C metadata=99cdadb944cbfccc -C extra-filename=-99cdadb944cbfccc --out-dir D:\a\rust\rust\build\i686-pc-windows-gnu\stage1-rustc\release\deps -L dependency=D:\a\rust\rust\build\i686-pc-windows-gnu\stage1-rustc\release\deps --extern proc_macro_hack=D:\a\rust\rust\build\i686-pc-windows-gnu\stage1-rustc\release\deps\proc_macro_hack-4b1e6edf13150f97.dll --extern quote=D:\a\rust\rust\build\i686-pc-windows-gnu\stage1-rustc\release\deps\libquote-86fa47d5a22c8465.rlib --extern syn=D:\a\rust\rust\build\i686-pc-windows-gnu\stage1-rustc\release\deps\libsyn-513900d7f78bc0a5.rlib --extern unic_langid_impl=D:\a\rust\rust\build\i686-pc-windows-gnu\stage1-rustc\release\deps\libunic_langid_impl-4f3e471d6f782a3b.rlib --extern proc_macro --cap-lints allow -Z binary-dep-depinfo` (exit code: 0xc0000005, STATUS_ACCESS_VIOLATION)
Build completed unsuccessfully in 0:16:55
Build completed unsuccessfully in 0:16:55
make: *** [Makefile:85: ci-mingw-subset-2] Error 1

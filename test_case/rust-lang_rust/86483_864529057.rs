
********************************************************************************
Regression in nightly-2021-02-14
********************************************************************************

fetching https://static.rust-lang.org/dist/2021-02-13/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2021-02-13: 40 B / 40 B [===========================================================================================================================] 100.00 % 218.71 KB/s converted 2021-02-13 to 3f5aee2d5241139d808f4fdece0026603489afd1
fetching https://static.rust-lang.org/dist/2021-02-14/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2021-02-14: 40 B / 40 B [===========================================================================================================================] 100.00 % 221.77 KB/s converted 2021-02-14 to 8e54a21139ae96a2aca3129100b057662e2799b9
looking for regression commit between 2021-02-13 and 2021-02-14
cloning rust repository
fetching (via local git) commits from 3f5aee2d5241139d808f4fdece0026603489afd1 to 8e54a21139ae96a2aca3129100b057662e2799b9
opening existing repository at "rust.git"
refreshing repository
looking up first commit
looking up second commit
checking that commits are by bors and thus have ci artifacts...
finding bors merge commits
found 7 bors merge commits in the specified range
  commit[0] 2021-02-12UTC: Auto merge of #81744 - rylev:overlapping-early-exit2, r=lcnr
  commit[1] 2021-02-13UTC: Auto merge of #82045 - Dylan-DPC:rollup-244l0sb, r=Dylan-DPC
  commit[2] 2021-02-13UTC: Auto merge of #82053 - JohnTitor:rollup-ymi9q0g, r=JohnTitor
  commit[3] 2021-02-13UTC: Auto merge of #81854 - the8472:specialize-clone-slice, r=Mark-Simulacrum
  commit[4] 2021-02-13UTC: Auto merge of #81666 - hyd-dev:miri-windows-test-fail, r=Mark-Simulacrum
  commit[5] 2021-02-13UTC: Auto merge of #81494 - cuviper:btree-node-init, r=Mark-Simulacrum
  commit[6] 2021-02-13UTC: Auto merge of #81238 - RalfJung:copy-intrinsics, r=m-ou-se
validated commits found, specifying toolchains

installing 3f5aee2d5241139d808f4fdece0026603489afd1
cargo for x86_64-unknown-linux-gnu: 5.97 MB / 5.97 MB [================================================================================================================] 100.00 % 7.18 MB/s testing...
RESULT: 3f5aee2d5241139d808f4fdece0026603489afd1, ===> No
uninstalling 3f5aee2d5241139d808f4fdece0026603489afd1

installing 8e54a21139ae96a2aca3129100b057662e2799b9
cargo for x86_64-unknown-linux-gnu: 5.96 MB / 5.96 MB [================================================================================================================] 100.00 % 7.38 MB/s testing...
RESULT: 8e54a21139ae96a2aca3129100b057662e2799b9, ===> Yes
uninstalling 8e54a21139ae96a2aca3129100b057662e2799b9

installing 3158857297417566824631a85c4cb3c0615ec6c2
cargo for x86_64-unknown-linux-gnu: 5.97 MB / 5.97 MB [================================================================================================================] 100.00 % 7.27 MB/s testing...
RESULT: 3158857297417566824631a85c4cb3c0615ec6c2, ===> Yes
uninstalling 3158857297417566824631a85c4cb3c0615ec6c2

installing 21cbbdc44de84e3ea99bca239091e5d1c49af654
cargo for x86_64-unknown-linux-gnu: 5.97 MB / 5.97 MB [================================================================================================================] 100.00 % 7.12 MB/s testing...
RESULT: 21cbbdc44de84e3ea99bca239091e5d1c49af654, ===> No
uninstalling 21cbbdc44de84e3ea99bca239091e5d1c49af654

installing dd4851d503f3fae0c0c742a19e0d8e6e2140bd2a
cargo for x86_64-unknown-linux-gnu: 5.97 MB / 5.97 MB [================================================================================================================] 100.00 % 7.58 MB/s testing...
RESULT: dd4851d503f3fae0c0c742a19e0d8e6e2140bd2a, ===> Yes
uninstalling dd4851d503f3fae0c0c742a19e0d8e6e2140bd2a

searched toolchains 3f5aee2d5241139d808f4fdece0026603489afd1 through 8e54a21139ae96a2aca3129100b057662e2799b9


********************************************************************************
Regression in dd4851d503f3fae0c0c742a19e0d8e6e2140bd2a
********************************************************************************

==================================================================================
= Please file this regression report on the rust-lang/rust GitHub repository     =
=        New issue: https://github.com/rust-lang/rust/issues/new                 =
=     Known issues: https://github.com/rust-lang/rust/issues                     =
= Copy and paste the text below into the issue report thread.  Thanks!           =
==================================================================================

searched nightlies: from nightly-2021-02-06 to nightly-2021-06-20
regressed nightly: nightly-2021-02-14
searched commits: from https://github.com/rust-lang/rust/commit/3f5aee2d5241139d808f4fdece0026603489afd1 to https://github.com/rust-lang/rust/commit/8e54a21139ae96a2aca3129100b057662e2799b9
regressed commit: https://github.com/rust-lang/rust/commit/dd4851d503f3fae0c0c742a19e0d8e6e2140bd2a

<details>
<summary>bisected with <a href='https://github.com/rust-lang/cargo-bisect-rustc'>cargo-bisect-rustc</a> v0.6.0</summary>


Host triple: x86_64-unknown-linux-gnu
Reproduce with:

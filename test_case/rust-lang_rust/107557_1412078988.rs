
searched toolchains nightly-2021-09-27 through nightly-2021-09-30


********************************************************************************
Regression in nightly-2021-09-28
********************************************************************************

fetching https://static.rust-lang.org/dist/2021-09-27/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2021-09-27: 40 B / 40 B [=========================================================================================================================] 100.00 % 344.14 KB/s converted 2021-09-27 to 05044c2e6c043929a11537d7f6169eb3a2397bb8
fetching https://static.rust-lang.org/dist/2021-09-28/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2021-09-28: 40 B / 40 B [=========================================================================================================================] 100.00 % 396.34 KB/s converted 2021-09-28 to 98c8619502093f34ca82f8f26ccf32e753924440
looking for regression commit between 2021-09-27 and 2021-09-28
fetching (via remote github) commits from max(05044c2e6c043929a11537d7f6169eb3a2397bb8, 2021-09-25) to 98c8619502093f34ca82f8f26ccf32e753924440
ending github query because we found starting sha: 05044c2e6c043929a11537d7f6169eb3a2397bb8
get_commits_between returning commits, len: 8
  commit[0] 2021-09-26UTC: Auto merge of #89144 - sexxi-goose:insig_stdlib, r=nikomatsakis
  commit[1] 2021-09-26UTC: Auto merge of #89092 - bjorn3:sync_cg_clif-2021-09-19, r=bjorn3
  commit[2] 2021-09-27UTC: Auto merge of #89145 - rusticstuff:bump_stdarch, r=kennytm
  commit[3] 2021-09-27UTC: Auto merge of #89182 - GuillaumeGomez:boostrap-explicit-request, r=Mark-Simulacrum
  commit[4] 2021-09-27UTC: Auto merge of #89203 - GuillaumeGomez:cleanup-rustdoc-types, r=camelid
  commit[5] 2021-09-27UTC: Auto merge of #89263 - TaKO8Ki:suggest-both-immutable-and-mutable-trait-implementations, r=estebank
  commit[6] 2021-09-27UTC: Auto merge of #89285 - jackh726:issue-88862, r=nikomatsakis
  commit[7] 2021-09-27UTC: Auto merge of #89214 - smoelius:register_tool, r=petrochenkov
ERROR: no CI builds available between 05044c2e6c043929a11537d7f6169eb3a2397bb8 and 98c8619502093f34ca82f8f26ccf32e753924440 within last 167 day

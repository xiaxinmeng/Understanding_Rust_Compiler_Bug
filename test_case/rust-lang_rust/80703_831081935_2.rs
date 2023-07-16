text
<snip individual tests>

searched toolchains nightly-2020-04-08 through nightly-2020-12-31

********************************************************************************
Regression in nightly-2020-05-23
********************************************************************************

fetching https://static.rust-lang.org/dist/2020-05-22/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2020-05-22: 40 B / 40 B [=================] 100.00 % 8.71 KB/s converted 2020-05-22 to 9310e3bd4f425f84fc27878ebf2bda1f30935a63
fetching https://static.rust-lang.org/dist/2020-05-23/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2020-05-23: 40 B / 40 B [==============] 100.00 % 1011.98 KB/s converted 2020-05-23 to 215f2d3294b08dbdcf8f7d40de21ef1e7eae0a2d
looking for regression commit between 2020-05-22 and 2020-05-23
opening existing repository at "../repos/rust/"
refreshing repository
fetching (via local git) commits from 9310e3bd4f425f84fc27878ebf2bda1f30935a63 to 215f2d3294b08dbdcf8f7d40de21ef1e7eae0a2d
opening existing repository at "../repos/rust/"
refreshing repository
looking up first commit
looking up second commit
checking that commits are by bors and thus have ci artifacts...
finding bors merge commits
found 7 bors merge commits in the specified range
  commit[0] 2020-05-21UTC: Auto merge of #71930 - Nadrieril:exhaustiveness-remove-tyerr, r=varkor
  commit[1] 2020-05-21UTC: Auto merge of #72433 - RalfJung:rollup-srft8nx, r=RalfJung
  commit[2] 2020-05-22UTC: Auto merge of #71956 - ecstatic-morse:remove-requires-storage-analysis, r=tmandry
  commit[3] 2020-05-22UTC: Auto merge of #72000 - cuviper:dist-llvm, r=Mark-Simulacrum
  commit[4] 2020-05-22UTC: Auto merge of #72458 - RalfJung:rollup-g1w1vws, r=RalfJung
  commit[5] 2020-05-22UTC: Auto merge of #72460 - RalfJung:rollup-28fs06y, r=RalfJung
  commit[6] 2020-05-22UTC: Auto merge of #72464 - RalfJung:rollup-xhm7w7u, r=RalfJung
ERROR: no commits between 9310e3bd4f425f84fc27878ebf2bda1f30935a63 and 215f2d3294b08dbdcf8f7d40de21ef1e7eae0a2d within last 167 days

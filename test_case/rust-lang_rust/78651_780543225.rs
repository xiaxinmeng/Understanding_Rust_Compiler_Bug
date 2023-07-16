
********************************************************************************
Regression in nightly-2020-06-25
********************************************************************************

fetching https://static.rust-lang.org/dist/2020-06-24/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2020-06-24: 40 B / 40 B [================] 100.00 % 674.64 KB/s converted 2020-06-24 to ff5b446d2fdbd898bc97a751f2f72858de185cf1
fetching https://static.rust-lang.org/dist/2020-06-25/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2020-06-25: 40 B / 40 B [================] 100.00 % 689.42 KB/s converted 2020-06-25 to 67100f61e62a86f2bf9e38552ee138e231eddc74
looking for regression commit between 2020-06-24 and 2020-06-25
opening existing repository at "rust.git"
refreshing repository
fetching (via local git) commits from ff5b446d2fdbd898bc97a751f2f72858de185cf1 to 67100f61e62a86f2bf9e38552ee138e231eddc74
opening existing repository at "rust.git"
refreshing repository
looking up first commit
looking up second commit
checking that commits are by bors and thus have ci artifacts...
finding bors merge commits
found 6 bors merge commits in the specified range
  commit[0] 2020-06-23UTC: Auto merge of #73644 - ollie27:rustdoc_alias_filter, r=GuillaumeGomez
  commit[1] 2020-06-23UTC: Auto merge of #73669 - Manishearth:rollup-0n4u7vq, r=Manishearth
  commit[2] 2020-06-24UTC: Auto merge of #73293 - Aaron1011:feature/macro-rules-arg-capture, r=petrochenkov
  commit[3] 2020-06-24UTC: Auto merge of #73679 - ehuss:update-cargo, r=ehuss
  commit[4] 2020-06-24UTC: Auto merge of #73692 - Dylan-DPC:rollup-ehzsbfw, r=Dylan-DPC
  commit[5] 2020-06-24UTC: Auto merge of #73660 - flip1995:clippyup, r=nikomatsakis
ERROR: no commits between ff5b446d2fdbd898bc97a751f2f72858de185cf1 and 67100f61e62a86f2bf9e38552ee138e231eddc74 within last 167 days

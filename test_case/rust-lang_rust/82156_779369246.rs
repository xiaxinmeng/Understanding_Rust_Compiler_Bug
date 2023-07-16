
********************************************************************************
Regression in nightly-2020-06-05
********************************************************************************

fetching https://static.rust-lang.org/dist/2020-06-04/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2020-06-04: 40 B / 40 B [===============] 100.00 % 720.31 KB/s converted 2020-06-04 to 56daaf669ebc3d5083db5cded719f780dc31104e
fetching https://static.rust-lang.org/dist/2020-06-05/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2020-06-05: 40 B / 40 B [===============] 100.00 % 778.59 KB/s converted 2020-06-05 to 47c3158c3d797f75f0f7b2b2a977179668919dab
looking for regression commit between 2020-06-04 and 2020-06-05
opening existing repository at "rust.git"
refreshing repository
fetching (via local git) commits from 56daaf669ebc3d5083db5cded719f780dc31104e to 47c3158c3d797f75f0f7b2b2a977179668919dab
opening existing repository at "rust.git"
refreshing repository
looking up first commit
looking up second commit
checking that commits are by bors and thus have ci artifacts...
finding bors merge commits
found 6 bors merge commits in the specified range
  commit[0] 2020-06-03UTC: Auto merge of #72948 - Dylan-DPC:rollup-fazhw00, r=Dylan-DPC
  commit[1] 2020-06-03UTC: Auto merge of #72754 - lcnr:predicate-fold, r=nikomatsakis
  commit[2] 2020-06-04UTC: Auto merge of #72618 - Aaron1011:feature/early-sourcemap, r=petrochenkov
  commit[3] 2020-06-04UTC: Auto merge of #72975 - Dylan-DPC:rollup-6zvco5x, r=Dylan-DPC
  commit[4] 2020-06-04UTC: Auto merge of #72882 - marmeladema:save-analysis-hir-tree, r=Xanewok
  commit[5] 2020-06-04UTC: Auto merge of #72995 - Dylan-DPC:rollup-7gsyb8x, r=Dylan-DPC
ERROR: no commits between 56daaf669ebc3d5083db5cded719f780dc31104e and 47c3158c3d797f75f0f7b2b2a977179668919dab within last 167 days

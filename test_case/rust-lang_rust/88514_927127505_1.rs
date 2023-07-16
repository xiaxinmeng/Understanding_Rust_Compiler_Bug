
********************************************************************************
Regression in nightly-2021-03-03
********************************************************************************

fetching https://static.rust-lang.org/dist/2021-03-02/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2021-03-02: 40 B / 40 B [=======================================================================================] 100.00 % 692.44 KB/s converted 2021-03-02 to 4f20caa6258d4c74ce6b316fd347e3efe81cf557
fetching https://static.rust-lang.org/dist/2021-03-03/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2021-03-03: 40 B / 40 B [=======================================================================================] 100.00 % 893.43 KB/s converted 2021-03-03 to 35dbef235048f9a2939dc20effe083ca483c37ff
looking for regression commit between 2021-03-02 and 2021-03-03
opening existing repository at "/Users/weihanglo/wd/contrib/rust"
refreshing repository
fetching (via local git) commits from 4f20caa6258d4c74ce6b316fd347e3efe81cf557 to 35dbef235048f9a2939dc20effe083ca483c37ff
opening existing repository at "/Users/weihanglo/wd/contrib/rust"
refreshing repository
looking up first commit
looking up second commit
checking that commits are by bors and thus have ci artifacts...
finding bors merge commits
found 6 bors merge commits in the specified range
  commit[0] 2021-03-01UTC: Auto merge of #82663 - jyn514:rollup-xh3cb0c, r=jyn514
  commit[1] 2021-03-02UTC: Auto merge of #82688 - GuillaumeGomez:rollup-b754t11, r=GuillaumeGomez
  commit[2] 2021-03-02UTC: Auto merge of #82634 - osa1:osa1/remove_old_fixme, r=Mark-Simulacrum
  commit[3] 2021-03-02UTC: Auto merge of #82698 - JohnTitor:rollup-htd533c, r=JohnTitor
  commit[4] 2021-03-02UTC: Auto merge of #82043 - tmiasko:may-have-side-effect, r=kennytm
  commit[5] 2021-03-02UTC: Auto merge of #82562 - llogiq:one-up-82248, r=oli-obk
ERROR: no commits between 4f20caa6258d4c74ce6b316fd347e3efe81cf557 and 35dbef235048f9a2939dc20effe083ca483c37ff within last 167 days
-----------------

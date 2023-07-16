
RUST_SRC_REPO=/home/nixon/upstreams/rust/rust cargo bisect-rustc --start=2020-01-01 --preserve --regress ice
installing nightly-2020-01-01
testing...
RESULT: nightly-2020-01-01, ===> No

installing nightly-2020-12-27
testing...
RESULT: nightly-2020-12-27, ===> Yes

installing nightly-2020-06-29
testing...
RESULT: nightly-2020-06-29, ===> Yes

installing nightly-2020-03-31
testing...
RESULT: nightly-2020-03-31, ===> No

installing nightly-2020-05-15
testing...
RESULT: nightly-2020-05-15, ===> Yes

installing nightly-2020-04-22
testing...
RESULT: nightly-2020-04-22, ===> No

installing nightly-2020-05-03
testing...
RESULT: nightly-2020-05-03, ===> Yes

installing nightly-2020-04-27
testing...
RESULT: nightly-2020-04-27, ===> No

installing nightly-2020-04-30
testing...
RESULT: nightly-2020-04-30, ===> No

installing nightly-2020-05-01
testing...
RESULT: nightly-2020-05-01, ===> No

installing nightly-2020-05-02
cargo for x86_64-unknown-linux-gnu: 5.00 MB / 5.00 MB [======================================================================] 100.00 % 4.14 MB/s testing...
RESULT: nightly-2020-05-02, ===> No

searched toolchains nightly-2020-01-01 through nightly-2020-12-27


********************************************************************************
Regression in nightly-2020-05-03
********************************************************************************

fetching https://static.rust-lang.org/dist/2020-05-02/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2020-05-02: 40 B / 40 B [=================================================================================] 100.00 % 716.15 KB/s converted 2020-05-02 to 7f65393b9abf5e70d0b9a8080558f17c5625bd40
fetching https://static.rust-lang.org/dist/2020-05-03/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2020-05-03: 40 B / 40 B [=================================================================================] 100.00 % 899.08 KB/s converted 2020-05-03 to f05a5240440b3eaef1684a7965860fab40301947
looking for regression commit between 2020-05-02 and 2020-05-03
opening existing repository at "/home/nixon/upstreams/rust/rust"
refreshing repository
fetching (via local git) commits from 7f65393b9abf5e70d0b9a8080558f17c5625bd40 to f05a5240440b3eaef1684a7965860fab40301947
opening existing repository at "/home/nixon/upstreams/rust/rust"
refreshing repository
looking up first commit
looking up second commit
checking that commits are by bors and thus have ci artifacts...
finding bors merge commits
found 9 bors merge commits in the specified range
  commit[0] 2020-05-01UTC: Auto merge of #71759 - Dylan-DPC:rollup-5hncork, r=Dylan-DPC
  commit[1] 2020-05-01UTC: Auto merge of #69808 - cjgillot:vtbl, r=pnkfelix
  commit[2] 2020-05-02UTC: Auto merge of #71776 - Dylan-DPC:rollup-k1iuuow, r=Dylan-DPC
  commit[3] 2020-05-02UTC: Auto merge of #71716 - alexcrichton:bitcode-follow-up, r=nnethercote
  commit[4] 2020-05-02UTC: Auto merge of #70170 - eddyb:wf-early-exit, r=nikomatsakis
  commit[5] 2020-05-02UTC: Auto merge of #71795 - RalfJung:rollup-yqxfi5a, r=RalfJung
  commit[6] 2020-05-02UTC: Auto merge of #70655 - oli-obk:subrepo_funness, r=Mark-Simulacrum
  commit[7] 2020-05-02UTC: Auto merge of #71794 - RalfJung:miri, r=RalfJung
  commit[8] 2020-05-02UTC: Auto merge of #69274 - LeSeulArtichaut:target-feature-11, r=hanna-kruppe
ERROR: no commits between 7f65393b9abf5e70d0b9a8080558f17c5625bd40 and f05a5240440b3eaef1684a7965860fab40301947 within last 167 days

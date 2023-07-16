
********************************************************************************
Regression in nightly-2020-10-07
********************************************************************************

fetching https://static.rust-lang.org/dist/2020-10-06/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2020-10-06: 40 B / 40 B [====================================================================] 100.00 % 746.12 KB/s converted 2020-10-06 to a1dfd2490a6cb456b92e469fa550dc217e20ad6d
fetching https://static.rust-lang.org/dist/2020-10-07/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2020-10-07: 40 B / 40 B [====================================================================] 100.00 % 724.12 KB/s converted 2020-10-07 to 98edd1fbf8a68977a2a7c1312eb1ebff80515a92
looking for regression commit between 2020-10-06 and 2020-10-07
fetching (via remote github) commits from max(a1dfd2490a6cb456b92e469fa550dc217e20ad6d, 2020-10-04) to 98edd1fbf8a68977a2a7c1312eb1ebff80515a92
ending github query because we found starting sha: a1dfd2490a6cb456b92e469fa550dc217e20ad6d
get_commits_between returning commits, len: 6
  commit[0] 2020-10-05UTC: Auto merge of #77080 - richkadel:llvm-coverage-counters-2, r=tmandry
  commit[1] 2020-10-06UTC: Auto merge of #77606 - JohnTitor:rollup-7rgahdt, r=JohnTitor
  commit[2] 2020-10-06UTC: Auto merge of #77594 - timvermeulen:chain_advance_by, r=scottmcm
  commit[3] 2020-10-06UTC: Auto merge of #73905 - matthewjasper:projection-bounds-2, r=nikomatsakis
  commit[4] 2020-10-06UTC: Auto merge of #76356 - caass:hooks, r=jyn514
  commit[5] 2020-10-06UTC: Auto merge of #77386 - joshtriplett:static-glibc, r=petrochenkov
ERROR: no CI builds available between a1dfd2490a6cb456b92e469fa550dc217e20ad6d and 98edd1fbf8a68977a2a7c1312eb1ebff80515a92 within last 167 days

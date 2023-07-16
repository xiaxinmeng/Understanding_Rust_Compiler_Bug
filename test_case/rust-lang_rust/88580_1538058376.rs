
********************************************************************************
Regression in nightly-2022-02-26
********************************************************************************

fetching https://static.rust-lang.org/dist/2022-02-25/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2022-02-25: 40 B / 40 B [===========================] 100.00 % 808.75 KB/s converted 2022-02-25 to 4b043faba34ccc053a4d0110634c323f6c03765e
fetching https://static.rust-lang.org/dist/2022-02-26/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2022-02-26: 40 B / 40 B [===========================] 100.00 % 775.05 KB/s converted 2022-02-26 to d3ad51b48f83329fac0cd8a9f1253f3146613c1c
looking for regression commit between 2022-02-25 and 2022-02-26
fetching (via remote github) commits from max(4b043faba34ccc053a4d0110634c323f6c03765e, 2022-02-23) to d3ad51b48f83329fac0cd8a9f1253f3146613c1c
ending github query because we found starting sha: 4b043faba34ccc053a4d0110634c323f6c03765e
get_commits_between returning commits, len: 11
  commit[0] 2022-02-24: Auto merge of #94131 - Mark-Simulacrum:fmt-string, r=oli-obk
  commit[1] 2022-02-24: Auto merge of #94333 - Dylan-DPC:rollup-7yxtywp, r=Dylan-DPC
  commit[2] 2022-02-25: Auto merge of #93368 - eddyb:diagbld-guarantee, r=estebank
  commit[3] 2022-02-25: Auto merge of #93878 - Aaron1011:newtype-macro, r=cjgillot
  commit[4] 2022-02-25: Auto merge of #94130 - erikdesjardins:partially, r=nikic
  commit[5] 2022-02-25: Auto merge of #94350 - matthiaskrgr:rollup-eesfiyr, r=matthiaskrgr
  commit[6] 2022-02-25: Auto merge of #93644 - michaelwoerister:simpler-debuginfo-typemap, r=wesleywiser
  commit[7] 2022-02-25: Auto merge of #94357 - matthiaskrgr:rollup-xrjaof3, r=matthiaskrgr
  commit[8] 2022-02-25: Auto merge of #94279 - tmiasko:write-print, r=Mark-Simulacrum
  commit[9] 2022-02-25: Auto merge of #94290 - Mark-Simulacrum:bump-bootstrap, r=pietroalbini
  commit[10] 2022-02-25: Auto merge of #94369 - matthiaskrgr:rollup-qtripm2, r=matthiaskrgr
ERROR: no CI builds available between 4b043faba34ccc053a4d0110634c323f6c03765e and d3ad51b48f83329fac0cd8a9f1253f3146613c1c within last 167 days

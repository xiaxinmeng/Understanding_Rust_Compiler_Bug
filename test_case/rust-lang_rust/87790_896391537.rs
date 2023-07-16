
********************************************************************************
Regression in nightly-2020-10-29
********************************************************************************

fetching https://static.rust-lang.org/dist/2020-10-28/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2020-10-28: 40 B / 40 B [==========================================================================================================] 100.00 % 755.88 KB/s converted 2020-10-28 to 07e968b640e8ff76fa8be4b48b70ab80ea577800
fetching https://static.rust-lang.org/dist/2020-10-29/channel-rust-nightly-git-commit-hash.txt
nightly manifest 2020-10-29: 40 B / 40 B [==========================================================================================================] 100.00 % 741.21 KB/s converted 2020-10-29 to 31ee872db5aae4750e3da1ca4ed1523c4356947f
looking for regression commit between 2020-10-28 and 2020-10-29
fetching (via remote github) commits from max(07e968b640e8ff76fa8be4b48b70ab80ea577800, 2020-10-26) to 31ee872db5aae4750e3da1ca4ed1523c4356947f
ending github query because we found starting sha: 07e968b640e8ff76fa8be4b48b70ab80ea577800
get_commits_between returning commits, len: 7
  commit[0] 2020-10-27UTC: Auto merge of #76269 - ayrtonm:function-reference-lint, r=oli-obk
  commit[1] 2020-10-27UTC: Auto merge of #75671 - nathanwhit:cstring-temp-lint, r=oli-obk
  commit[2] 2020-10-28UTC: Auto merge of #78458 - Dylan-DPC:rollup-tan044s, r=Dylan-DPC
  commit[3] 2020-10-28UTC: Auto merge of #78323 - est31:smaller_list_overlap, r=varkor
  commit[4] 2020-10-28UTC: Auto merge of #78409 - pietroalbini:build-manifest-checksum-cache, r=Mark-Simulacrum
  commit[5] 2020-10-28UTC: Auto merge of #78414 - nox:function-sections, r=nagisa,bjorn3
  commit[6] 2020-10-28UTC: Auto merge of #78415 - tgnottingham:expn_id_tag_hash, r=Aaron1011
ERROR: no commits between 07e968b640e8ff76fa8be4b48b70ab80ea577800 and 31ee872db5aae4750e3da1ca4ed1523c4356947f within last 167 days

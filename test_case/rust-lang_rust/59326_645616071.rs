
root@ubuntu-cloud-computing-tmp:~/bad_code# cargo bisect-rustc --start 2019-01-04 --end 2019-01-08 --with-src --with-cargo --verbose --preserve                                              
installing nightly-2019-01-04                                                                                                                                                                
testing...                                                                                                                                                                                   
RESULT: nightly-2019-01-04, ===> No                                                                                                                                                          
                                                                                                                                                                                             
installing nightly-2019-01-08                                                                                                                                                                
testing...                                                                                                                                                                                   
RESULT: nightly-2019-01-08, ===> Yes                                                                                                                                                         
                                                                                                                                                                                             
installing nightly-2019-01-06                                                                                                                                                                
testing...                                                                                                                                                                                   
RESULT: nightly-2019-01-06, ===> Yes                                                                                                                                                         
                                                                                                                                                                                             
installing nightly-2019-01-05                                                                                                                                                                
testing...                                                                                                                                                                                   
RESULT: nightly-2019-01-05, ===> No                                                                                                                                                          
                                                                                                                                                                                             
searched toolchains nightly-2019-01-04 through nightly-2019-01-08                                                                                                                            
                                                                                                                                                                                             
                                                                                                                                                                                             
********************************************************************************                                                                                                             
Regression in nightly-2019-01-06                                                                                                                                                             
********************************************************************************                                                                                                             
                                                                                                                                                                                             
fetching https://static.rust-lang.org/dist/2019-01-05/channel-rust-nightly-git-commit-hash.txt                                                                                               
nightly manifest 2019-01-05: 40 B / 40 B [============================================================================================================================] 100.00 % 376.15 KB/s converted 2019-01-05 to f381a962550436f74dd6e9021e4df2fdefb96cfa
fetching https://static.rust-lang.org/dist/2019-01-06/channel-rust-nightly-git-commit-hash.txt 
nightly manifest 2019-01-06: 40 B / 40 B [===========================================================================================================================] 100.00 % 1006.09 KB/s 
converted 2019-01-06 to 68fe5182c967259ef89dbe313e4bf80f45a53e7e
looking for regression commit between 2019-01-05 and 2019-01-06
opening existing repository at "rust.git"
refreshing repository
fetching (via local git) commits from f381a962550436f74dd6e9021e4df2fdefb96cfa to 68fe5182c967259ef89dbe313e4bf80f45a53e7e
opening existing repository at "rust.git"
refreshing repository
looking up first commit
looking up second commit
checking that commits are by bors and thus have ci artifacts...
finding bors merge commits
found 8 bors merge commits in the specified range
  commit[0] 2019-01-04UTC: Auto merge of #56897 - euclio:parse-fatal, r=estebank
  commit[1] 2019-01-04UTC: Auto merge of #56079 - mark-i-m:patch-1, r=nikomatsakis
  commit[2] 2019-01-05UTC: Auto merge of #56145 - weiznich:re_rebalance_coherence, r=nikomatsakis
  commit[3] 2019-01-05UTC: Auto merge of #56837 - arielb1:nonprincipal-trait-objects, r=nikomatsakis
  commit[4] 2019-01-05UTC: Auto merge of #57099 - davidtwco:issue-57098, r=nikomatsakis
  commit[5] 2019-01-05UTC: Auto merge of #57101 - o01eg:fix-57014, r=alexcrichton
  commit[6] 2019-01-05UTC: Auto merge of #57145 - RalfJung:panic-if-uninhabited, r=alexcrichton
  commit[7] 2019-01-05UTC: Auto merge of #57354 - kennytm:rollup, r=kennytm
ERROR: no commits between f381a962550436f74dd6e9021e4df2fdefb96cfa and 68fe5182c967259ef89dbe313e4bf80f45a53e7e within last 167 days

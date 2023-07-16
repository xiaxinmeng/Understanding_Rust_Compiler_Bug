plain
2020-03-02T00:04:11.4324981Z ========================== Starting Command Output ===========================
2020-03-02T00:04:11.4328043Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/2bb7fb09-aff7-494a-8f90-7da8c847a38e.sh
2020-03-02T00:04:11.4328370Z 
2020-03-02T00:04:11.4332264Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-02T00:04:11.4351844Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69625/merge to s
2020-03-02T00:04:11.4355354Z Task         : Get sources
2020-03-02T00:04:11.4355695Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-02T00:04:11.4355976Z Version      : 1.0.0
2020-03-02T00:04:11.4356169Z Author       : Microsoft
---
2020-03-02T00:04:12.6028594Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-02T00:04:12.6040292Z ##[command]git config gc.auto 0
2020-03-02T00:04:12.6046364Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-02T00:04:12.6056015Z ##[command]git config --get-all http.proxy
2020-03-02T00:04:12.6069202Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69625/merge:refs/remotes/pull/69625/merge
---
2020-03-02T00:09:33.9981762Z     Checking core v0.0.0 (/checkout/src/libcore)
2020-03-02T00:09:40.1638735Z error[E0520]: `nth` specializes an item from a parent `impl`, but that item is not marked `default`
2020-03-02T00:09:40.1639980Z    --> src/libcore/iter/adapters/mod.rs:351:5
2020-03-02T00:09:40.1642042Z     |
2020-03-02T00:09:40.1643857Z 315 | / impl<'a, I, T: 'a> Iterator for Cloned<I>
2020-03-02T00:09:40.1644767Z 316 | | where
2020-03-02T00:09:40.1646716Z 317 | |     I: Iterator<Item = &'a T>,
2020-03-02T00:09:40.1648366Z ...   |
2020-03-02T00:09:40.1649027Z 344 | |     }
2020-03-02T00:09:40.1649778Z 345 | | }
2020-03-02T00:09:40.1650686Z     | |_- parent `impl` is here
2020-03-02T00:09:40.1650686Z     | |_- parent `impl` is here
2020-03-02T00:09:40.1651224Z ...
2020-03-02T00:09:40.1651954Z 351 | /     fn nth(&mut self, n: usize) -> Option<T> {
2020-03-02T00:09:40.1653023Z 352 | |         self.it.nth(n).cloned()
2020-03-02T00:09:40.1654206Z 353 | |     }
2020-03-02T00:09:40.1655019Z     | |_____^ cannot specialize default item `nth`
2020-03-02T00:09:40.1655617Z     |
2020-03-02T00:09:40.1656351Z     = note: to specialize, `nth` in the parent `impl` must be marked `default`
2020-03-02T00:09:40.1657433Z error[E0520]: `last` specializes an item from a parent `impl`, but that item is not marked `default`
2020-03-02T00:09:40.1658721Z    --> src/libcore/iter/adapters/mod.rs:355:5
2020-03-02T00:09:40.1659279Z     |
2020-03-02T00:09:40.1659279Z     |
2020-03-02T00:09:40.1660009Z 315 | / impl<'a, I, T: 'a> Iterator for Cloned<I>
2020-03-02T00:09:40.1660853Z 316 | | where
2020-03-02T00:09:40.1662081Z 317 | |     I: Iterator<Item = &'a T>,
2020-03-02T00:09:40.1663684Z ...   |
2020-03-02T00:09:40.1664500Z 344 | |     }
2020-03-02T00:09:40.1665441Z 345 | | }
2020-03-02T00:09:40.1666340Z     | |_- parent `impl` is here
2020-03-02T00:09:40.1666340Z     | |_- parent `impl` is here
2020-03-02T00:09:40.1666929Z ...
2020-03-02T00:09:40.1667631Z 355 | /     fn last(self) -> Option<T> {
2020-03-02T00:09:40.1668485Z 356 | |         self.it.last().cloned()
2020-03-02T00:09:40.1669285Z 357 | |     }
2020-03-02T00:09:40.1670095Z     | |_____^ cannot specialize default item `last`
2020-03-02T00:09:40.1670693Z     |
2020-03-02T00:09:40.1671801Z     = note: to specialize, `last` in the parent `impl` must be marked `default`
2020-03-02T00:09:40.1672997Z error[E0520]: `count` specializes an item from a parent `impl`, but that item is not marked `default`
2020-03-02T00:09:40.1673723Z    --> src/libcore/iter/adapters/mod.rs:359:5
2020-03-02T00:09:40.1674206Z     |
2020-03-02T00:09:40.1674206Z     |
2020-03-02T00:09:40.1675066Z 315 | / impl<'a, I, T: 'a> Iterator for Cloned<I>
2020-03-02T00:09:40.1675874Z 316 | | where
2020-03-02T00:09:40.1676705Z 317 | |     I: Iterator<Item = &'a T>,
2020-03-02T00:09:40.1678274Z ...   |
2020-03-02T00:09:40.1679085Z 344 | |     }
2020-03-02T00:09:40.1679844Z 345 | | }
2020-03-02T00:09:40.1681361Z     | |_- parent `impl` is here
---
2020-03-02T00:09:45.0882645Z   local time: Mon Mar  2 00:09:45 UTC 2020
2020-03-02T00:09:45.3759517Z   network time: Mon, 02 Mar 2020 00:09:45 GMT
2020-03-02T00:09:45.3762991Z == end clock drift check ==
2020-03-02T00:09:52.5187076Z 
2020-03-02T00:09:52.5238345Z ##[error]Bash exited with code '1'.
2020-03-02T00:09:52.5253675Z ##[section]Finishing: Run build
2020-03-02T00:09:52.5302150Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69625/merge to s
2020-03-02T00:09:52.5308008Z Task         : Get sources
2020-03-02T00:09:52.5308400Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-02T00:09:52.5308746Z Version      : 1.0.0
2020-03-02T00:09:52.5308983Z Author       : Microsoft
2020-03-02T00:09:52.5308983Z Author       : Microsoft
2020-03-02T00:09:52.5309380Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-02T00:09:52.5309817Z ==============================================================================
2020-03-02T00:09:52.9116470Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-02T00:09:52.9163480Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69625/merge to s
2020-03-02T00:09:52.9330413Z Cleaning up task key
2020-03-02T00:09:52.9331777Z Start cleaning up orphan processes.
2020-03-02T00:09:52.9606738Z Terminate orphan process: pid (3431) (python)
2020-03-02T00:09:52.9823774Z ##[section]Finishing: Finalize Job

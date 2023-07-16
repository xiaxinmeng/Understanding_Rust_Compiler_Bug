plain
Looks like docker image is the same as before, not uploading
----> 61a941b8badbce727085c505068d72fa3e737f5b
++++++++
61a941b8badbce727085c505068d72fa3e737f5b--------
commit 14ee7975e49b7d761230305542833b8d8c72f642 Merge: 61a941b8 0b763e7a Author: Guillaume Gomez <guillaume1.gomez@gmail.com> Date: Thu Aug 5 18:50:17 2021 +0000 Merge 0b763e7a849243ee515fe64e2a0c8ece3a59d5e1 into 61a941b8badbce727085c505068d72fa3e737f5b commit 0b763e7a849243ee515fe64e2a0c8ece3a59d5e1 Author: Guillaume Gomez <guillaume1.gomez@gmail.com> Date: Thu Aug 5 15:21:52 2021 +0200 Remove git fetch command call commit 61a941b8badbce727085c505068d72fa3e737f5b Author: bors <bors@rust-lang.org> Date: Thu Aug 5 14:45:09 2021 +0000 Auto merge of #87737 - LeSeulArtichaut:unsafeck-less-freeze, r=oli-obk Only compute `is_freeze` for layout-constrained ADTs Places are usually shallow and quick to visit. By contrast, computing `is_freeze` can be much costlier, involving inference and trait solving. Making sure to call `is_freeze` only when necessary should be beneficial for performance in most cases. See [this comparison](https://perf.rust-lang.org/compare.html?start=81f08a4763e7537b92506fa5a597e6bf774d20cc&end=56a58d347b1c7dd0c2984b8fc3930c408e26fbc2&stat=instructions%3Au) from #87710. r? `@oli-obk`
##[error]Process completed with exit code 1.

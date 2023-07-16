
lunch-box. for x in stable beta nightly; do echo === $x ===; rustup update $x; /usr/bin/time rustc +$x foo.rs; done
=== stable ===
info: syncing channel updates for 'stable-x86_64-unknown-linux-gnu'

  stable-x86_64-unknown-linux-gnu unchanged - rustc 1.20.0 (f3d6973f4 2017-08-27)

5.97user 0.04system 0:06.02elapsed 100%CPU (0avgtext+0avgdata 121132maxresident)k
0inputs+7680outputs (0major+12912minor)pagefaults 0swaps
=== beta ===
info: syncing channel updates for 'beta-x86_64-unknown-linux-gnu'

  beta-x86_64-unknown-linux-gnu unchanged - rustc 1.21.0-beta.1 (198109911 2017-08-29)

9.99user 1.50system 0:11.49elapsed 100%CPU (0avgtext+0avgdata 387528maxresident)k
0inputs+8056outputs (0major+1408205minor)pagefaults 0swaps
=== nightly ===
info: syncing channel updates for 'nightly-x86_64-unknown-linux-gnu'

  nightly-x86_64-unknown-linux-gnu unchanged - rustc 1.21.0-nightly (97b01abf3 2017-08-31)

10.16user 1.58system 0:11.74elapsed 100%CPU (0avgtext+0avgdata 368428maxresident)k
0inputs+8008outputs (0major+1709226minor)pagefaults 0swaps

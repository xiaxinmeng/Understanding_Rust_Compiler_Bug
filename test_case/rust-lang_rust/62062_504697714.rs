plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:00db9d3b
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
Cloning into 'rust-lang/rust'...
travis_time:end:00db9d3b:start=1561236884528252885,finish=1561236890454781640,duration=5926528755
$ cd rust-lang/rust
$ git checkout -qf d15831326c415cfe964abe98915d89282a0db597
fatal: reference is not a tree: d15831326c415cfe964abe98915d89282a0db597
The command "git checkout -qf d15831326c415cfe964abe98915d89282a0db597" failed and exited with 128 during .
Your build has been stopped.

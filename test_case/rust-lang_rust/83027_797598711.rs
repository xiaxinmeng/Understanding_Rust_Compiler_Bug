
$ uname -a
Linux frink 5.11.4-arch1-1 #1 SMP PREEMPT Sun, 07 Mar 2021 18:00:49 +0000 x86_64 GNU/Linux
$ lscpu | rg Model
Model:                           79
Model name:                      Intel(R) Core(TM) i7-6900K CPU @ 3.20GHz
$ rustc --print target-cpus | rg native
    native         - Select the CPU of the current host (currently broadwell).
$ rustc +stable --version
rustc 1.50.0 (cb75ad5db 2021-02-10)
$ rustc +beta --version
rustc 1.51.0-beta.4 (4d25f4607 2021-03-05)
$ cd /tmp
$ git clone https://github.com/BurntSushi/ripgrep
$ cd ripgrep
$ git rev-parse HEAD
c7730d1f3a366e42fdd497a1e0db4bf090de415c

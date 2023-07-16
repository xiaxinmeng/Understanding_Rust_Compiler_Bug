
$ python x.py build
downloading https://static.rust-lang.org/dist/2022-11-01/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
#=#=-  #       #                                                                                                                                                                                                                                               #=O#-     #        #                                                                                                                                                                                                                                           -#O=- #      #          #         ######################################################################################################################################################################################################################################################### 100.0%
extracting /home/user/rust/build/cache/2022-11-01/rust-std-beta-x86_64-unknown-linux-gnu.tar.xz
downloading https://static.rust-lang.org/dist/2022-11-01/rustc-beta-x86_64-unknown-linux-gnu.tar.xz
#=#=-  #       #                                                                                                                                                                                                                                               #=O#-     #      ######################################################################################################################################################################################################################################################### 100.0%
extracting /home/user/rust/build/cache/2022-11-01/rustc-beta-x86_64-unknown-linux-gnu.tar.xz
downloading https://static.rust-lang.org/dist/2022-11-01/cargo-beta-x86_64-unknown-linux-gnu.tar.xz
######################################################################################################################################################################################################################################################### 100.0%
extracting /home/user/rust/build/cache/2022-11-01/cargo-beta-x86_64-unknown-linux-gnu.tar.xz
Building rustbuild
  Downloaded cpufeatures v0.2.2
  Downloaded crossbeam-utils v0.8.8
  Downloaded 2 crates (50.3 KB) in 0.26s
   Compiling libc v0.2.137
   Compiling memchr v2.5.0
   Compiling proc-macro2 v1.0.46
   Compiling cfg-if v1.0.0
   Compiling version_check v0.9.4
   Compiling unicode-ident v1.0.0
   Compiling typenum v1.15.0
   Compiling syn v1.0.102
   Compiling cc v1.0.73
   Compiling serde_derive v1.0.137
   Compiling lazy_static v1.4.0
   Compiling serde v1.0.137
   Compiling pkg-config v0.3.25
   Compiling io-lifetimes v1.0.1
   Compiling log v0.4.17
   Compiling regex-automata v0.1.10
   Compiling crossbeam-utils v0.8.8
   Compiling regex-syntax v0.6.26
   Compiling rustix v0.36.3
   Compiling fnv v1.0.7
   Compiling once_cell v1.12.0
   Compiling linux-raw-sys v0.1.3
   Compiling same-file v1.0.6
   Compiling bitflags v1.3.2
   Compiling serde_json v1.0.81
   Compiling ryu v1.0.10
   Compiling unicode-width v0.1.9
   Compiling cpufeatures v0.2.2
   Compiling itoa v1.0.2
   Compiling bootstrap v0.0.0 (/home/user/rust/src/bootstrap)
   Compiling walkdir v2.3.2
   Compiling getopts v0.2.21
   Compiling thread_local v1.1.4
   Compiling hex v0.4.3
   Compiling generic-array v0.14.5
   Compiling cmake v0.1.48
   Compiling lzma-sys v0.1.17
   Compiling bstr v0.2.17
   Compiling aho-corasick v0.7.18
   Compiling object v0.29.0
   Compiling quote v1.0.18
   Compiling opener v0.5.0
   Compiling xattr v0.2.3
   Compiling filetime v0.2.16
   Compiling tar v0.4.38
   Compiling crypto-common v0.1.3
   Compiling block-buffer v0.10.2
   Compiling digest v0.10.3
   Compiling regex v1.5.6
   Compiling sha2 v0.10.2
   Compiling globset v0.4.8
   Compiling ignore v0.4.18
   Compiling fd-lock v3.0.8
   Compiling xz2 v0.1.6
   Compiling toml v0.5.9
    Finished dev [unoptimized] target(s) in 10.28s

Couldn't find required command: cmake

You should install cmake, or set `download-ci-llvm = true` in the
`[llvm]` section section of `config.toml` to download LLVM rather
than building it.

Build completed unsuccessfully in 0:00:30

$ sudo apt install cmake
Reading package lists... Done
Building dependency tree... Done
Reading state information... Done
The following additional packages will be installed:
  cmake-data dh-elpa-helper libjsoncpp25 librhash0
Suggested packages:
  cmake-doc ninja-build cmake-format
The following NEW packages will be installed:
  cmake cmake-data dh-elpa-helper libjsoncpp25 librhash0
0 upgraded, 5 newly installed, 0 to remove and 44 not upgraded.
Need to get 7,138 kB of archives.
After this operation, 31.8 MB of additional disk space will be used.
Do you want to continue? [Y/n] 
Get:1 http://apt.pop-os.org/ubuntu jammy/main amd64 libjsoncpp25 amd64 1.9.5-3 [80.0 kB]
Get:2 http://apt.pop-os.org/ubuntu jammy/main amd64 librhash0 amd64 1.4.2-1ubuntu1 [125 kB]
Get:3 http://apt.pop-os.org/ubuntu jammy/main amd64 dh-elpa-helper all 2.0.9ubuntu1 [7,610 B]
Get:4 http://apt.pop-os.org/ubuntu jammy-updates/main amd64 cmake-data all 3.22.1-1ubuntu1.22.04.1 [1,913 kB]
Get:5 http://apt.pop-os.org/ubuntu jammy-updates/main amd64 cmake amd64 3.22.1-1ubuntu1.22.04.1 [5,013 kB]
Fetched 7,138 kB in 1s (9,489 kB/s)
Selecting previously unselected package libjsoncpp25:amd64.
(Reading database ... 282274 files and directories currently installed.)
Preparing to unpack .../libjsoncpp25_1.9.5-3_amd64.deb ...
Unpacking libjsoncpp25:amd64 (1.9.5-3) ...
Selecting previously unselected package librhash0:amd64.
Preparing to unpack .../librhash0_1.4.2-1ubuntu1_amd64.deb ...
Unpacking librhash0:amd64 (1.4.2-1ubuntu1) ...
Selecting previously unselected package dh-elpa-helper.
Preparing to unpack .../dh-elpa-helper_2.0.9ubuntu1_all.deb ...
Unpacking dh-elpa-helper (2.0.9ubuntu1) ...
Selecting previously unselected package cmake-data.
Preparing to unpack .../cmake-data_3.22.1-1ubuntu1.22.04.1_all.deb ...
Unpacking cmake-data (3.22.1-1ubuntu1.22.04.1) ...
Selecting previously unselected package cmake.
Preparing to unpack .../cmake_3.22.1-1ubuntu1.22.04.1_amd64.deb ...
Unpacking cmake (3.22.1-1ubuntu1.22.04.1) ...
Setting up dh-elpa-helper (2.0.9ubuntu1) ...
Setting up libjsoncpp25:amd64 (1.9.5-3) ...
Setting up librhash0:amd64 (1.4.2-1ubuntu1) ...
Setting up cmake-data (3.22.1-1ubuntu1.22.04.1) ...
Setting up cmake (3.22.1-1ubuntu1.22.04.1) ...
Processing triggers for man-db (2.10.2-1) ...
Processing triggers for libc-bin (2.35-0ubuntu3.1) ...

$ python x.py build
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.02s

Couldn't find required command: ninja (or ninja-build)

You should install ninja as described at
<https://github.com/ninja-build/ninja/wiki/Pre-built-Ninja-packages>,
or set `ninja = false` in the `[llvm]` section of `config.toml`.
Alternatively, set `download-ci-llvm = true` in that `[llvm]` section
to download LLVM rather than building it.

Build completed unsuccessfully in 0:00:00

$ sudo apt install ninja-build
Reading package lists... Done
Building dependency tree... Done
Reading state information... Done
The following NEW packages will be installed:
  ninja-build
0 upgraded, 1 newly installed, 0 to remove and 55 not upgraded.
Need to get 111 kB of archives.
After this operation, 313 kB of additional disk space will be used.
Get:1 http://apt.pop-os.org/release jammy/main amd64 ninja-build amd64 1.10.1-1pop0~1630348646~22.04~fe200e8 [111 kB]
Fetched 111 kB in 0s (686 kB/s) 
Selecting previously unselected package ninja-build.
(Reading database ... 285360 files and directories currently installed.)
Preparing to unpack .../ninja-build_1.10.1-1pop0~1630348646~22.04~fe200e8_amd64.deb ...
Unpacking ninja-build (1.10.1-1pop0~1630348646~22.04~fe200e8) ...
Setting up ninja-build (1.10.1-1pop0~1630348646~22.04~fe200e8) ...
Processing triggers for man-db (2.10.2-1) ...

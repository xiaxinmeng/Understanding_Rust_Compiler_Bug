
:; grep -v ^# config.toml | grep .
changelog-seen = 2
[llvm]
download-ci-llvm = false
[build]
[install]
prefix = "/pz/SFW"
[rust]
[target.x86_64-unknown-illumos]
cc = "/usr/gcc/9/bin/gcc"
cxx = "/usr/gcc/9/bin/g++"
ar = "/usr/gcc/9/bin/gcc-ar"
[dist]

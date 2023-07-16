
:; grep -E "(cc|cxx|ar) =" config.toml | grep -v ^#
cc = "/usr/gcc/9/bin/gcc"
cxx = "/usr/gcc/9/bin/g++"
ar = "/usr/gcc/9/bin/gcc-ar"

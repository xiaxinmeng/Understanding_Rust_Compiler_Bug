 rest
When you build Clang, you will need to give *it* access to modern C++11
standard library in order to use it as your new host in part of a bootstrap.
There are two easy ways to do this, either build (and install) libc++ along
with Clang and then use it with the ``-stdlib=libc++`` compile and link flag,
or install Clang into the same prefix (``$HOME/toolchains`` above) as GCC.
Clang will look within its own prefix for libstdc++ and use it if found. You
can also add an explicit prefix for Clang to look in for a GCC toolchain with
the ``--gcc-toolchain=/opt/my/gcc/prefix`` flag, passing it to both compile and
link commands when using your just-built-Clang to bootstrap.


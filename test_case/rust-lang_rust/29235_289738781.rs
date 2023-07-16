
# Using an i686-pc-windows-gnu host rustc
$ rustc foo.rs --emit obj
$ nm -g foo.o
00000000 T __ZN3foo3foo17h83583f8ff17ec21aE
# Using a x86_64-pc-windows-gnu host rustc
$ rustc foo.rs --emit obj
$ nm -g foo.o
0000000000000000 T _ZN3foo3foo17h83583f8ff17ec21aE

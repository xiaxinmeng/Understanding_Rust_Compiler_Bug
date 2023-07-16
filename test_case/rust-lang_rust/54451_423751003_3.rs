 console
$ rustc +stable --emit=obj -O foo.rs

$ nm -C foo.o
0000000000000000 t foo
0000000000000000 r .Lbyte_str.0
0000000000000000 T foo::bar

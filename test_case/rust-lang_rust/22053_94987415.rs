
$ rustc src/libstd/lib.rs --test -C opt-level=3 -C lto
<many warnings>
$ echo $?
0

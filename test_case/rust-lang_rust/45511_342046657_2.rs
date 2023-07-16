
$ rustc +nightly --crate-name test -g -Z thinlto --crate-type dylib foo.rs -O -C codegen-units=16 --emit llvm-ir
$ llc test.test0-*.ll -filetype=obj -O0 -o foo.o
$ dwarfdump -i foo.o > /dev/null

dwarfdump ERROR:  reference form with no valid local ref?!, offset=<0x00001496>:  DW_DLE_ATTR_FORM_OFFSET_BAD (119)

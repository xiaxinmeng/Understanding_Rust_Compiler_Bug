
$ rustc +nightly --crate-name test -g -Z thinlto --crate-type dylib foo.rs -O -C codegen-units=16 --emit obj
$ for x in *.o; do echo $x; dwarfdump -i $x > /dev/null; done
test.test0-8787f43e282added376259c1adb08b80.rs.rust-cgu.o

dwarfdump ERROR:  reference form with no valid local ref?!, offset=<0x0000131c>:  DW_DLE_ATTR_FORM_OFFSET_BAD (119)
test.test1-8787f43e282added376259c1adb08b80.rs.rust-cgu.o
test.test2-8787f43e282added376259c1adb08b80.rs.rust-cgu.o
test.test3-8787f43e282added376259c1adb08b80.rs.rust-cgu.o
test.test4-8787f43e282added376259c1adb08b80.rs.rust-cgu.o
test.test5-8787f43e282added376259c1adb08b80.rs.rust-cgu.o
test.test6-8787f43e282added376259c1adb08b80.rs.rust-cgu.o
test.test7-8787f43e282added376259c1adb08b80.rs.rust-cgu.o
test.test8-8787f43e282added376259c1adb08b80.rs.rust-cgu.o
test.test9-8787f43e282added376259c1adb08b80.rs.rust-cgu.o

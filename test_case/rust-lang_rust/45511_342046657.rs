
$ cat foo.rs
pub fn foo(a: &mut Vec<String>) {
    a.clone();
}
$ rustc +nightly --version
rustc 1.23.0-nightly (f0fe716db 2017-10-30)
$ rustc +nightly --crate-name test -g -Z thinlto --crate-type dylib foo.rs -O -C codegen-units=16
$ dwarfdump -i libtest.so > /dev/null
dwarfdump ERROR:  reference form with no valid local ref?!, offset=<0x0000131c>:  DW_DLE_ATTR_FORM_OFFSET_BAD (119)

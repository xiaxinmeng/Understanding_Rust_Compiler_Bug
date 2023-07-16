
$ cat > lib.rs
pub struct Foo(pub Vec<u8>);
impl Drop for Foo {
    fn drop(&mut self) {}
}
$ rustc +nightly --crate-type dylib lib.rs -Zshare-generics -Cprefer-dynamic
$ cat > main.rs
fn main() {
    lib::Foo(vec![]);
}
$ rustc +nightly --crate-type bin main.rs -Zshare-generics --extern lib=liblib.so -Cprefer-dynamic
$ objdump -d ./main | grep Foo
00000000000013c0 <_ZN4core3ptr29drop_in_place$LT$lib..Foo$GT$17ha6b7547f0832ce86E>:
    13c8:       48 8b 0d f9 2b 00 00    mov    0x2bf9(%rip),%rcx        # 3fc8 <_ZN50_$LT$lib..Foo$u20$as$u20$core..ops..drop..Drop$GT$4drop17hcc626ef64eb19045E>
    13d1:       eb 00                   jmp    13d3 <_ZN4core3ptr29drop_in_place$LT$lib..Foo$GT$17ha6b7547f0832ce86E+0x13>
    13dc:       eb 27                   jmp    1405 <_ZN4core3ptr29drop_in_place$LT$lib..Foo$GT$17ha6b7547f0832ce86E+0x45>
    13e7:       eb 10                   jmp    13f9 <_ZN4core3ptr29drop_in_place$LT$lib..Foo$GT$17ha6b7547f0832ce86E+0x39>
    13f7:       eb e5                   jmp    13de <_ZN4core3ptr29drop_in_place$LT$lib..Foo$GT$17ha6b7547f0832ce86E+0x1e>
    1531:       e8 8a fe ff ff          callq  13c0 <_ZN4core3ptr29drop_in_place$LT$lib..Foo$GT$17ha6b7547f0832ce86E>

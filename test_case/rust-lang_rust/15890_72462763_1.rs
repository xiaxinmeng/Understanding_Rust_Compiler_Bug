 bash
/tmp $ mkdir $(echo 'foo\xff')
/tmp $ touch $(echo 'foo\xff')/foo.rs
/tmp $ echo "fn main() {}" > $(echo 'foo\xff')/foo.rs
/tmp $ rustc $(echo 'foo\xff')/foo.rs

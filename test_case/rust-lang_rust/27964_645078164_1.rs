
<source>:9:1: 13:2 error: the trait `core::marker::Sized` is not implemented for the type `Unsized + 'static` [E0277]
<source>: 9 impl OopsSized<Unsized> for Foo {
<source>:10     fn foo(&self) -> Option<Box<Unsized>> {
<source>:11         None
<source>:12     }
<source>:13 }
<source>:9:1: 13:2 help: run `rustc --explain E0277` to see a detailed explanation
<source>:9:1: 13:2 note: `Unsized + 'static` does not have a constant size known at compile-time
<source>: 9 impl OopsSized<Unsized> for Foo {
<source>:10     fn foo(&self) -> Option<Box<Unsized>> {
<source>:11         None
<source>:12     }
<source>:13 }

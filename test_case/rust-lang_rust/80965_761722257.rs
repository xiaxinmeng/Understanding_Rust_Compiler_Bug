rust
// check-pass
// run-rustfix

#![feature(doc_notable_trait)]

#[doc(spotlight)]
//~^ WARN `doc(spotlight)` was renamed to `doc(notable_trait)`
trait MyTrait {}

mod my_module {
    #[doc(spotlight)]
    //~^ WARN `doc(spotlight)` was renamed to `doc(notable_trait)`
    use crate::MyTrait;
}

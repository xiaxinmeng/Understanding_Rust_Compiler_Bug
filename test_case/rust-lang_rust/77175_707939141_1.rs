
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
// check-pass
// edition:2018

// error: lifetime parameter `'a` only used once
#[deny(single_use_lifetimes)]
fn foo<'a>(s1: String, s2: &str, s3: &'a str) -> &'a str { s3 }

async fn bar<'a, '_>(s1: String, s2: &'_ str, s3: &'a str)
 ->
     /*impl Trait*/ #[lang = "from_generator"](move |mut _task_context|
                                                   {
                                                       let s1 = s1;
                                                       let s2 = s2;
                                                       let s3 = s3;
                                                       { let _t = { s3 }; _t }
                                                   })

fn main() { }

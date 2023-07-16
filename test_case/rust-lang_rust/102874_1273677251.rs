bash
$ cat a.rs                                                                                                    
pub struct A {
    pub a_field: bool,
}

$ rustc +nightly --edition-2021 --crate-type rlib a.rs                                                        

$ cat b.rs 
pub use a::A;

$ rustdoc +nightly --edition=2021 b.rs --extern a=liba.rlib -Zunstable-options -wjson --document-private-items

$ grep a_field doc/b.json # no output

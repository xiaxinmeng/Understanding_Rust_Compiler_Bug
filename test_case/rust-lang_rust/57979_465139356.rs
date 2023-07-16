rust
#![allow(dead_code)]

pub trait Quux { type Assoc; }
pub trait Foo<T> { }
pub trait Bar { }

struct S1;
struct S2;
struct S3;
struct S4;

impl Quux for S1 { type Assoc = S2; }
impl Foo<S3> for S2 { }
impl Bar for S3 { }
impl Bar for S4 { }

// The presence/absence of this line dictates whether
// we get inference failures or not
impl Foo<S4> for S2 { }

pub fn foobar(_: impl Quux<Assoc=impl Foo<impl Bar>>) {

}

fn main() {
    foobar(S1);
}

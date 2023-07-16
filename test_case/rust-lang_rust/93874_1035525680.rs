rust
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

pub trait Build {
    type Output<O>;
    fn build<O>(self, input: O) -> Self::Output<O>;
}

pub struct IdentityBuild;
impl Build for IdentityBuild {
    type Output<O> = O;
    fn build<O>(self, input: O) -> Self::Output<O> {
        input
    }
}

fn x() {
    let mut f = IdentityBuild.build(|| ());
    (f)();
    // ^type annotations needed
    // type must be known at this pointrustcE0282
    // main.rs(17, 9): consider giving `x` a type
}
fn y() {
    type MyFn = impl FnOnce();
    let f: MyFn = IdentityBuild.build(|| ());
    (f)();
    // Compiles
}

pub fn main() {
    x();
}

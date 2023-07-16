
//
// (I.e. the v2 assertion passes, and the v0, v1, and v3 Debug::fmts are all
// overriden by the Access trait below.)

fn main() {
    let foo0 = Foo_v0("v0");
    let foo1 = Foo_v1("v1");
    let foo2 = Foo_v2("v2");
    let foo3 = Foo_v3("v3");

    std::panic::catch_unwind(|| assert_eq!("Foo_v0(\"v0\")", format!("{:?}", foo0)));
    std::panic::catch_unwind(|| assert_eq!("Foo_v1(\"v1\")", format!("{:?}", foo1)));
    std::panic::catch_unwind(|| assert_eq!("Foo_v2(\"v2\")", format!("{:?}", foo2)));
    std::panic::catch_unwind(|| assert_eq!("Foo_v3(\"v3\")", format!("{:?}", foo3)));
}


// This is where behavior is changing between stable and nightly
#[derive(Debug)]
pub struct Foo_v0<T>(pub T);

// On all of v1+v2+v3, stable+beta+nightly have same behavior.

pub struct Foo_v1<T>(pub T);
pub struct Foo_v2<T>(pub T);
pub struct Foo_v3<T>(pub T);

// This, v1, most closely matches the first part of the current expansion of
// `derive(Debug)` that we see in v0.
//
// The weird thing is: Why do we see different behavior here than for Foo_v0 on
// stable? (On *nightly*, v0 and v1 have the same behavior.)
impl <T: fmt::Debug> fmt::Debug for Foo_v1<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Foo_v1(ref __self_0_0) => {
                let mut debug_trait_builder = f.debug_tuple("Foo_v1");
                let _ = debug_trait_builder.field(&&(*__self_0_0));
                debug_trait_builder.finish()
            }
        }
    }
}

// This adds a `&mut` borrow of the DebugTuple returned by the formatter, which
// effectively stops it from being accidentally overriden by Access trait below.
//
// (I.e. it seems to be using the same mechanism that `&mut fmt::Formatter` uses
//  to achieve robustness in the face of such accidental method collisions from
//  traits.)
impl <T: fmt::Debug> fmt::Debug for Foo_v2<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Foo_v2(ref __self_0_0) => {
                let mut debug_trait_builder = f.debug_tuple("Foo_v2");
                let _ = (&mut debug_trait_builder).field(&&(*__self_0_0));
                debug_trait_builder.finish()
            }
        }
    }
}

// This adds an explicit deref of the formatter before invoking debug_tuple.
//
// Doing so makes the formatter vulnerable to the accidental method collision:
// It now resolves to Access::debug_tuple, below, on stable+beta+nightly.
//
// This variant is an attempt to explain the source of the robustness that we
// observe when using `&mut`: the presence of `&mut` forces the method resolver
// to use an extra deref when looking up `debug_tuple`, and that, for some
// reason, makes it favor the inherent `&mut self` method over the `&self`
// method in `Access::debug_tuple`.
impl <T: fmt::Debug> fmt::Debug for Foo_v3<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Foo_v3(ref __self_0_0) => {
                let mut debug_trait_builder = (*f).debug_tuple("Foo_v3");
                let _ = debug_trait_builder.field(&&(*__self_0_0));
                debug_trait_builder.finish()
            }
        }
    }
}

impl<T> Access for T {}
pub trait Access {
    fn debug_tuple(&self, x: &str) -> fmt::DebugTuple {
        panic!("got into debug_tuple on {:?}", x);
    }

    fn field(&self, x: impl fmt::Debug) {
        panic!("got into field on {:?}", x);
    }
}

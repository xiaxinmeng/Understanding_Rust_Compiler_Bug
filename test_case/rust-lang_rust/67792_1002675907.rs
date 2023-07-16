rust
#![feature(
    const_fn_floating_point_arithmetic,
    const_fn_trait_bound,
    // The next one is necessary to prevent E0658, at least currently
    const_refs_to_cell,
    const_trait_impl
)]

use std::cmp::Ordering;

const fn const_min<T>(a: T, b: T) -> T
where
    T: ~const PartialEq + ~const PartialOrd + ~const Drop,
{
    if a <= b {
        a
    } else {
        b
    }
}

#[derive(PartialEq, PartialOrd)]
struct NotConst {
    f: f32,
}

struct Const {
    f: f32,
}

impl const PartialEq for Const {
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f
    }
}

impl const PartialOrd for Const {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.f < other.f {
            Some(Ordering::Less)
        } else if self.f == other.f {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Greater)
        }
    }
}

fn main() {
    // Works normally / as expected at runtime, without `const` impls.
    let f = NotConst { f: 12.0 };
    let g = NotConst { f: 24.0 };
    println!("{}", const_min(f, g).f);

    // Works normally / as expected at compile-time, with `const` impls.
    const F: Const = Const { f: 12.0 };
    const G: Const = Const { f: 24.0 };
    const H: f32 = const_min(F, G).f;
    println!("{}", H);

    // Works normally / as expected at runtime on primitive numeric variables.
    let ff = 12.0;
    let gg = 24.0;
    println!("{}", const_min(ff, gg));

    // Does not work at compile-time on primitive numeric constants! Uncomment
    // the next four lines to see the error messages.
    // const FF: f32 = 12.0;
    // const GG: f32 = 24.0;
    // const HH: f32 = const_min(FF, GG);
    // println!("{}", HH);
}

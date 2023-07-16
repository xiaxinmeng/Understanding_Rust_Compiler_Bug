Rust
#![crate_type="rlib"]

trait A {
    fn k(&self, x: i32) -> i32;
}

pub struct O {
}

pub struct P {
}

impl A for O {
    fn k(&self, x: i32) -> i32 {
        x
    }
}

impl A for P {
    fn k(&self, x: i32) -> i32 {
        x
    }
}

pub enum R {
    P(P),
    O(O),
}

// hack to work around the lack of generic closures, we don't
// want to use a trait object here because performance is a
// problem.
macro_rules! with_inner {
    ($this:expr, $var:ident => $body:expr) => {
        match $this {
            &R::P(ref $var) => { $body }
            &R::O(ref $var) => { $body }
        }
    }
}

impl R {
    pub fn k(&self, x: i32) -> i32 {
        match self {
            &R::P(ref p) => p.k(x),
            &R::O(ref o) => o.k(x),
        }
    }

    pub fn j(&self, x: i32) -> i32 {
        with_inner!(self, a => a.k(x))
    }
}

// Marker to allow seeing the assembly easily
#[no_mangle]
pub fn j_exported(r: &R, x: i32) -> i32 { r.j(x) }

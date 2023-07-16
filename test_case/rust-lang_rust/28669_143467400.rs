 Rust
pub trait Lift<'tcx> {
    type Lifted /* adding :'tcx solves this */;
}

pub fn lift<'tcx,T:?Sized+Lift<'tcx>>(value: &T) -> T::Lifted {
    loop {}
}

pub fn my_lift_to_tcx<'tcx, T: Lift<'tcx>>(this: &T)
{   
    let mut result = Vec::new(); // adding a type annotation here solves this
    result.push(lift(this));
}

rust
trait Id<T> {
    fn id(&self) -> &T;
}

impl<T> Id<T> for T {
    fn id(&self) -> &T { self }
}

trait SyncSetup<T> {
    type From: Id<T> + Sync + 'static;
}

fn cast<T, U>(from: &T) -> &(dyn Id<U> + Sync) {
    fn do_it<T, U: ?Sized + SyncSetup<T>>(val: &U::From) -> &(dyn Id<T> + Sync) {
        val
    }
    
    do_it::<U, dyn SyncSetup<U, From=T>>(from)
}

fn syncify<T>(from: &T) -> &'static (dyn Id<T> + Sync) {
    // Very carefully avoid ICE. This only adds marker trait to an unsized trait.
    let x: &(dyn Id<T> + Sync) = cast(from);
    // Indirect over another reference and make `x` appear `&'static`.
    // Lifetime change is invisible to trait materialization checks.
    let y: &(dyn Id<&'static (dyn Id<T> + Sync)>) = cast(&x);
    // .. and grab that one.
    let x: &'static (dyn Id<T> + Sync) = *(y.id());
    x
}

use std::cell::Cell;
fn main() {
    let st = Cell::new(0u32);
    let x = syncify(&st);
    
    std::thread::spawn(move || {
        x.id().set(1);
    });
    
    while st.get() == 0 {}
    println!("Spooky");
}

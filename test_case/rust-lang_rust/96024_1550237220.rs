rust
/// std:

// from variation 4.)
#[impls_may_provide]
trait Error {
  ...
}

//// User crate:

struct MyError {
   backtrace: Backtrace  
}

impl MyError {
    // attribute / macro magic that turns this into `[(TypeId<Backtrace>, usize)]`. I.e. field offsets
    // variation 1.)
    #[provides] 
    const PROVIDES = [Self::backtrace];

    // variation 2.) a)
    #[provides] 
    const PROVIDES = [get_backtrace];

    fn get_backtrace(&self) -> &Backtrace {...}
}

impl Error for MyError {}

// Variation 2.) b)
#[provides]
impl HasBacktrace for MyError {
   fn backtrace(&self) -> ... { ... }
}



fn foo(err: &dyn Error) {
   // get concrete type, variation 1.)
   let bt: Option<&Backtrace> = core::any::request::<Backtrace>(&err);
   // Variation 2.) b)
   let bt: Option<&dyn HasBacktrace> = core::any::sidecast::<HasBacktrace>(&err);
}

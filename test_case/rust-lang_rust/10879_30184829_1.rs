
struct Local;
struct Managed { marker: Local } // inheritance-ish!
struct Gc<T> { val: ..., marker: Managed }
trait NoManaged: DoesNotContain<Managed> { } // (+ impl)
trait Send: 'static + DoesNotContain<Local> { } // (+ impl)
struct ReferenceCounted<T> { marker: Local }

struct Mutable;
struct &mut 'a T { ..., marker: Mutable } // imaginary syntax, this would have to be wired-in (sadly)
struct Cell<T> { ..., marker: Mutable }
struct RefCell<T> { ..., marker: Mutable }
trait Freeze: DoesNotContain<Mutable> { } // (+ impl)

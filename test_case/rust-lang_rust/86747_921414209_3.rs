
// Reader: okay, this clearly returns a scope guard
fn make_guard1() -> Box<dyn Drop> { ... }

// Reader: why is the drop deferred for this? is the lifetime of the guard supposed to outlast something?
// Reader: *looks up definition of DeferredDrop*
// Reader: oh, I misunderstood the signature to be more significant than it really is
//         (which could happen even if it's named something like ScopeGuard)
fn make_guard2() -> Box<dyn DeferredDrop> { ... }

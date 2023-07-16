rust
struct NotSync(*mut ());

trait AssociatedSync {
    type ShouldBeSync: Sync;
}

struct Dummy;

impl AssociatedSync /* span error points here */ for Dummy {
    type ShouldBeSync = NotSync /* whereas it should probably point here */;
}

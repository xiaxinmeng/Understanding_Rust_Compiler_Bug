rust
trait IsDynSync {}
impl IsDynSync for dyn '_ + Sync {}

fn ref_id<T : ?Sized + IsDynSync> (
    it: &'_ T,
) -> &'_ T
{
    it
}

fn main ()
{
    // Current behavior: OK
    let _: &'_ dyn Sync = ref_id(&());
    // Imitation of the expected behavior: Error
    let _: &'_ dyn Sync = ref_id(&()) as _;
}

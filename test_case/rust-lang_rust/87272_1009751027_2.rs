rust
struct Wrapper<T>(T);
trait NeedTryInto: Sized where Wrapper<Self>: TryInto<Self> {}
fn foo<NTI: NeedTryInto>(_: NTI) {}

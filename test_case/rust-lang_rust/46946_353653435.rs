rust
trait Equal<Other>
where Other: Equal<Self>
{}

struct A {}
struct B {}

impl Equal<B> for A {}
impl Equal<A> for B {}

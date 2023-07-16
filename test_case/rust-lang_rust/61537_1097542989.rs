rust
trait Tr<const B: bool> {}
struct Ty;

impl Tr<true> for Ty {}
impl Tr<false> for Ty {}

struct Test<const B: bool, T: Tr<B>>(T);
struct Test2<const B: bool>(Test<B, Ty>);

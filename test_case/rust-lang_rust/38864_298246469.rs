rust
struct SomeType;
struct SomeOtherType;

trait SomeTrait {
    type AssocType;
}
trait SomeOtherTrait {}

impl SomeTrait for SomeType where Self::AssocType: SomeOtherTrait {
    type AssocType = SomeOtherType;
}
fn main() {}

rust
trait TraitWithAssociatedType {
    type AssociatedType;
}
trait AnyTrait { }
struct StructWithImpl;
struct AnyStruct;

impl TraitWithAssociatedType for StructWithImpl where Self::AssociatedType: AnyTrait {
    type AssociatedType = AnyStruct;
}

rs
// error "stability attributes may not be used outside of the standard library" ignored
// .. etc
impl<#[unstable(feature = "allocator_api", issue = "32838")] A: Allocator>
    ::core::marker::StructuralEq for String<A>
{
}
// .. etc (None of the other impls have the stability annotation)

rust
trait SomeTraitWithOtherAssociated<'a> {
    type Associated: SomeOtherTrait<'a>;
}

impl<'a, T: SomeTrait<'a>> SomeTraitWithOtherAssociated<'a> for T
where
    for<'b> <T as SomeTrait<'b>>::Associated: SomeOtherTrait<'b>,
{
    type Associated = <T as SomeTrait<'a>>::Associated;
}

fn something_else<T>(v: T)
where
    for<'b> T: SomeTraitWithOtherAssociated<'b>,
{
    // ...

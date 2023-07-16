rust
// current impl
impl<'a, Meta: Copy + MyTrait> Copy for FwdIter<'a, Meta>
    where Meta::AssocType: 'a { }

// required impl
impl<'a, Meta: Copy + MyTrait> Copy for FwdIter<'a, Meta>
    where Meta::AssocType: Copy + 'a { }

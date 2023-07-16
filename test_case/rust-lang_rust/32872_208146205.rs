
impl<'a, Meta: MetaAssocTypes> Copy for FwdIter<'a, Meta>
    where Meta::AssocType: 'a {}

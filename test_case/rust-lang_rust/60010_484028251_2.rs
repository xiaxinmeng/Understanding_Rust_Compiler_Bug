rust
struct SalsaStorage { _parse: <ParseQuery as Query<RootDatabase>>::Data, }
// ...
impl<DB> Query<DB> for ParseQuery where DB: SourceDatabase { type Data = RootDatabase; }

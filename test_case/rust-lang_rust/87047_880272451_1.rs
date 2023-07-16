rust
pub fn run_recursive<I>(&self, it: I) -> RecursiveResults
where
    I: IntoIterator<Item = result::Result<DirEntry, Error>>,
{
    let (ents, errs): (Vec<_>, Vec<_>) = it.collect();
    RecursiveResults { ents, errs }
}

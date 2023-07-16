rust
pub fn run_recursive<I>(&self, it: I) -> RecursiveResults
where
    I: IntoIterator<Item = result::Result<DirEntry, Error>>,
{
    let mut results = RecursiveResults { ents: vec![], errs: vec![] };
    for result in it {
        match result {
            Ok(ent) => results.ents.push(ent),
            Err(err) => results.errs.push(err),
        }
    }
    results
}

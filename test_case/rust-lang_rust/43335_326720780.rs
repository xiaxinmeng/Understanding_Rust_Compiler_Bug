rust
fn anaconda<I: Iterator>(mut it: I) {
    let _: Result<(),()> = do catch {
        loop {
            it.next().ok_or(())?;
        }
    };
}

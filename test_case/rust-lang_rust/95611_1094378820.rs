rust
pub fn parse_file<P, T>(p: P) -> Result<Vec<T>, Box<dyn Error>>
where
    P: AsRef<Path>,
    T: FromStr<Err = Box<dyn Error>>,
    <T as FromStr>::Err: Error + 'static,
{
    let contents = read_file(p)?;

    parse_raw_data(contents)
}

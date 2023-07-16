rust
impl str {
    pub fn trim_start(&self) -> &str;

    pub fn trim_end(&self) -> &str;

    pub fn trim_start_matches<'a, P>(&'a self, pat: P) -> &'a str
    where
        P: Pattern<'a>;

    pub fn trim_end_matches<'a, P>(&'a self, pat: P) -> &'a str
    where
        P: Pattern<'a>,
        P::Searcher: ReverseSearcher<'a>;
}

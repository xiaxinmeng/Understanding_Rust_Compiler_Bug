
//Querying
fn len(&self) -> usize
fn is_empty(&self) -> bool
fn width(&self, is_cjk: bool) -> usize
fn is_char_boundary(&self, index: usize) -> bool


//Slicing and char retrieval
fn as_bytes(&self) -> &[u8]
fn as_ptr(&self) -> *const u8
unsafe fn slice_unchecked(&self, begin: usize, end: usize) -> &str
fn slice_chars(&self, begin: usize, end: usize) -> &str
fn char_range_at(&self, start: usize) -> CharRange
fn char_range_at_reverse(&self, start: usize) -> CharRange
fn char_at(&self, i: usize) -> char
fn char_at_reverse(&self, i: usize) -> char
fn slice_shift_char(&self) -> Option<(char, &str)>

//Iterators
fn chars(&self) -> Chars
fn char_indices(&self) -> CharIndices
fn bytes(&self) -> Bytes
fn words(&self) -> Words
fn lines(&self) -> Lines
fn lines_any(&self) -> LinesAny
fn nfd_chars(&self) -> Decompositions
fn nfkd_chars(&self) -> Decompositions
fn nfc_chars(&self) -> Recompositions
fn nfkc_chars(&self) -> Recompositions
fn graphemes(&self, is_extended: bool) -> Graphemes
fn grapheme_indices(&self, is_extended: bool) -> GraphemeIndices
fn utf16_units(&self) -> Utf16Units


//Searching
fn contains<'a, P>(&'a self, pat: P) -> bool where P: Pattern<'a>
fn starts_with<'a, P>(&'a self, pat: P) -> bool where P: Pattern<'a>
fn ends_with<'a, P>(&'a self, pat: P) -> bool where P: Pattern<'a>, P::Searcher: ReverseSearcher<'a>
fn find<'a, P>(&'a self, pat: P) -> Option<usize> where P: Pattern<'a>
fn rfind<'a, P>(&'a self, pat: P) -> Option<usize> where P: Pattern<'a>, P::Searcher: ReverseSearcher<'a>
fn split<'a, P>(&'a self, pat: P) -> Split<'a, P> where P: Pattern<'a>
fn rsplit<'a, P>(&'a self, pat: P) -> RSplit<'a, P> where P: Pattern<'a>, P::Searcher: ReverseSearcher<'a>
fn split_terminator<'a, P>(&'a self, pat: P) -> SplitTerminator<'a, P> where P: Pattern<'a>
fn rsplit_terminator<'a, P>(&'a self, pat: P) -> RSplitTerminator<'a, P> where P: Pattern<'a>, P::Searcher: ReverseSearcher<'a>
fn splitn<'a, P>(&'a self, count: usize, pat: P) -> SplitN<'a, P> where P: Pattern<'a>
fn rsplitn<'a, P>(&'a self, count: usize, pat: P) -> RSplitN<'a, P> where P: Pattern<'a>, P::Searcher: ReverseSearcher<'a>
fn matches<'a, P>(&'a self, pat: P) -> Matches<'a, P> where P: Pattern<'a>
fn rmatches<'a, P>(&'a self, pat: P) -> RMatches<'a, P> where P: Pattern<'a>, P::Searcher: ReverseSearcher<'a>
fn match_indices<'a, P>(&'a self, pat: P) -> MatchIndices<'a, P> where P: Pattern<'a>
fn rmatch_indices<'a, P>(&'a self, pat: P) -> RMatchIndices<'a, P> where P: Pattern<'a>, P::Searcher: ReverseSearcher<'a>

fn subslice_offset(&self, inner: &str) -> usize


//Trim
fn trim(&self) -> &str
fn trim_left(&self) -> &str
fn trim_right(&self) -> &str

fn trim_matches<'a, P>(&'a self, pat: P) -> &'a str where P: Pattern<'a>, P::Searcher: DoubleEndedSearcher<'a>
fn trim_left_matches<'a, P>(&'a self, pat: P) -> &'a str where P: Pattern<'a>
fn trim_right_matches<'a, P>(&'a self, pat: P) -> &'a str where P: Pattern<'a>, P::Searcher: ReverseSearcher<'a>


//Conversion
fn parse<F>(&self) -> Result<F, F::Err> where F: FromStr

fn to_lowercase(&self) -> String
fn to_uppercase(&self) -> String
fn escape_default(&self) -> String
fn escape_unicode(&self) -> String
fn replace(&self, from: &str, to: &str) -> String


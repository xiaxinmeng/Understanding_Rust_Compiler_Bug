 rust
/// Convert a char to its uppercase equivalent
///
/// The case-folding performed is the common or simple mapping:
/// it maps one unicode codepoint (one char in Rust) to its uppercase equivalent according
/// to the Unicode database at ftp://ftp.unicode.org/Public/UNIDATA/UnicodeData.txt
/// The additional SpecialCasing.txt is not considered here, as it expands to multiple
/// codepoints in some cases.
///
/// A full reference can be found here
/// http://www.unicode.org/versions/Unicode4.0.0/ch03.pdf#G33992
///
/// # Return value
///
/// Returns the char itself if no conversion was made
#[inline]
pub fn to_uppercase(c: char) -> char {
    conversions::to_upper(c)
}
/// Convert a char to its lowercase equivalent
///
/// The case-folding performed is the common or simple mapping
/// see `to_uppercase` for references and more information
///
/// # Return value
///
/// Returns the char itself if no conversion if possible
#[inline]
pub fn to_lowercase(c: char) -> char {
    conversions::to_lower(c)
}

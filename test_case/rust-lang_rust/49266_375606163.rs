rust
// Does not implement `PartialEq`
struct ResolvedIdent {
    // True identity of the identifier, was previously obtained by hygienic resolution
    def_id: DefId,
    // Purely informational string, should never be compared, but can be e.g. displayed in error messages
    info: InternedString,
}

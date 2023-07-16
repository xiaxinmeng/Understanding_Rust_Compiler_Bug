rust
enum SyntaxContextData {
    Local(/* the current SyntaxContextData fields */),
    Remote(CrateNum, SyntaxContext)
}

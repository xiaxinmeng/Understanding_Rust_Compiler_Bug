
pub enum TokenStream {
    Empty,
    Tree(TokenTree, IsJoint),
    Stream(Lrc<Vec<TokenStream>>),
}

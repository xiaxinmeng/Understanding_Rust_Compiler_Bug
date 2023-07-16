rust
#[non_exhaustive] // !
enum MacroCallContext {
    Expression, // Statement?
    Pattern,
    Type,
    Statement, // Item? Expression?
    Item, // Statement?
    TraitItem, // Item?
    ImplItem, // Item?
    ForeignItem, // Item?
    Unknown, // !
}

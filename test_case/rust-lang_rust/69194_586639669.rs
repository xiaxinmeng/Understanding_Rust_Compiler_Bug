rust
enum ItemKind {
    Fn(FnCommon),
    Const(ConstCommon),
    TyAlias(TyAliasCommon),
    Mac(MacCommon),
    Static(StaticCommon),
    ... // Others
}

enum AssocItemKind {
    Fn(FnCommon),
    Const(ConstCommon),
    TyAlias(TyAliasCommon),
    Mac(MacCommon),
}

enum ForeignItemKind {
    Fn(FnCommon),
    TyAlias(TyAliasCommon),
    Mac(MacCommon),
    Static(StaticCommon),
}

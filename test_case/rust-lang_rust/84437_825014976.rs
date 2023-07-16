rust
enum Cfg {
    True,
    False,
    Cfg(Symbol, Option<Symbol>),
    Not(Box<Cfg>),
    Any(Vec<Cfg>),
    All(Vec<Cfg>),
}

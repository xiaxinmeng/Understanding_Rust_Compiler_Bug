rust
enum DefPathDataName {
    Named(Symbol),
    Anon { namespace: Symbol },
}
impl DefPathData {
    fn name(&self) -> DefPathDataName {
        match self {
            DefPathData::ValueNs(name) | ... => DefPathDataName::Name(name),
            DefPathData::ClosureExpr => DefPathDataName::Anon { namespace: sym::closure },
            ...
        }
    }
}

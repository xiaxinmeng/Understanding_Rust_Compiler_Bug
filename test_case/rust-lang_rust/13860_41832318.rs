 rust
enum NS {
    StructNS,
    ValNS,
    TypeNS,
    ModNS,
}

struct Subscope {
    table: SmallIntMap<NS, DefId>,
}

impl Subscope {
    fn new() -> Subscope {
        Subscope { table: SmallIntMap::with_capacity(num_variants::<NS>()) }
    }
}

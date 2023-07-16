 rust
#[deriving(Bounded)]
enum NS {
    StructNS = 3, // because maybe it's C compat?
    ValNS,
    TypeNS,
    ModNS,
}

struct Subscope {
    table: SmallIntMap<NS, DefId>,
}

impl Subscope {
    fn new() -> Subscope {
        let (min, max): (NS, NS) = (Bounded::min_value(), Bounded::max_value());
        let count = max as uint - min as uint + 1;
        Subscope { table: SmallIntMap::with_capacity(count) }
    }
}

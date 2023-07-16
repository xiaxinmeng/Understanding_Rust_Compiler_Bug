rs
pub struct PidPool {
    pool: BTreeSet<u32>,
}

impl PidPool {
    pub const fn new() -> PidPool {
        PidPool {
            pool: BTreeSet::new(),
        }
    }
}

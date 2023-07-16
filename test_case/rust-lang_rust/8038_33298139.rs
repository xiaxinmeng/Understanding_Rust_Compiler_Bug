
pub trait Hasher {
    fn result_u64(&mut self) -> u64;
}

pub trait StreamHasher: Hasher {
    fn input(&mut self, bytes: &[u8]);
}

pub trait Hashable<H: Hasher> {
    fn hash2(&self, h: &mut H);
}

impl<H: StreamHasher> Hashable<H> for i8 { ... }

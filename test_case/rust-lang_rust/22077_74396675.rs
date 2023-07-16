 rust
impl<K, Q: ?Sized, V, S, H> Index<Q> for HashMap<K, V, S>
    where K: Eq + Hash<H>,
          Q: Eq + Hash<H> + BorrowFrom<K>,
          S: HashState<Hasher=H>,
          H: hash::Hasher<Output=u64>
{
    type Output = V;

    #[inline]
    fn index<'a>(&'a self, index: &Q) -> &'a V {
        self.get(index).expect("no entry found for key")
    }
}

impl<K, V, S, H, Q: ?Sized> IndexMut<Q> for HashMap<K, V, S>
    where K: Eq + Hash<H>,
          Q: Eq + Hash<H> + BorrowFrom<K>,
          S: HashState<Hasher=H>,
          H: hash::Hasher<Output=u64>
{
    #[inline]
    fn index_mut<'a>(&'a mut self, index: &Q) -> &'a mut V {
        self.get_mut(index).expect("no entry found for key")
    }
}

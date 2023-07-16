
pub trait Component: ThreadSync + 'static {
	type Storage: Map<Key = LocalVersion, Val = Self> + Index<LocalVersion, Output=Self> + IndexMut<LocalVersion, Output=Self> + ThreadSync + 'static;
}

impl<C> Component for C where C: ThreadSync + 'static {
	default type Storage = SecondaryMap<LocalVersion, Self>;
}

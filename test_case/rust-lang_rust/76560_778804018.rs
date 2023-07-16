
[u8 ; size_assert::<HeapStorage, T>()] : Sized =>
[u8 ; HeapStorage::Asserter<T>::VALUE as usize - 1] : Sized =>
[u8 ; AlwaysTrueAsserter::VALUE as usize - 1] : Sized =>
[u8 ; true as usize - 1] : Sized =>
[u8 ; 0] : Sized


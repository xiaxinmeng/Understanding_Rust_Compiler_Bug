
pub struct AHashMap<K,V,A> {
    allocator: A,
    ...
}

pub type HashMap<K,V,A> = AHashMap<K,V,Heap>;

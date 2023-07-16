 rust
struct MapChain<'this, 'next, K, V> { 
    ... 
    next: Option<&'this mut MapChain<'next, K, V>> 
}

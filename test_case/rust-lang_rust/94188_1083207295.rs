rust
#[macro_export]
macro_rules! define_id_collections {
    ($map_name:ident, $set_name:ident, $stable_map_name:ident, $stable_set_name:ident, $key:ty) => {
        pub type $map_name<T> = $crate::fx::FxHashMap<$key, T>;
        pub type $set_name = $crate::fx::FxHashSet<$key>;
        pub type $stable_map_name<T> = $crate::stable_set::StableMap<$key, T>;  
        pub type $stable_set_name = $crate::stable_set::StableSet<$key>;
    };
}

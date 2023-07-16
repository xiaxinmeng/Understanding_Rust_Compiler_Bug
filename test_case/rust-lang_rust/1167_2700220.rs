
mod c_vec {
    type t<T> = { base: *T, size: uint };
    fn create<T>(base: *T, size: uint) -> @t<T> { ret @{ base:base, size:size}; }
    fn get<T>(v: t<T>, idx: uint) -> T unsafe { assert idx < v.size; ret v.base[idx]; }
}  

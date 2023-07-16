 rust
fn get_compile_unit_metadata(cache: metadata_cache, full_path: str)
    -> @metadata<compile_unit_md> unsafe {
    ret alt *cache[0] {
      compile_unit_metadata(md) { unsafe::reinterpret_cast(md) }
    };
}


/// The common prefix of all vtables. It is followed by function pointers for trait methods.
///
/// Private implementation detail of `DynMetadata::size_of` etc.
#[repr(C)]
struct VTable {
    drop_in_place: fn(*mut ()),
    size_of: usize,
    align_of: usize,
}

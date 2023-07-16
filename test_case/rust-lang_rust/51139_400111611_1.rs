rust
/// True if this def-id refers to the implicit constructor for a tuple struct like `struct Foo(u32)`
pub fn is_struct_constructor(self, def_id: DefId) -> bool {
        self.def_key(def_id).disambiguated_data.data == DefPathData::StructCtor
}

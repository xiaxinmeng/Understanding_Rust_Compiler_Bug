
// &T (regular reference)
DW_TAG_reference_type
    DW_AT_rust_pointer_kind  thin
    DW_AT_mutable            true or false // defaults to false if not present
    DW_AT_type               <ref to type>

// &[T] (regular slice)
DW_TAG_reference_type
    DW_AT_rust_pointer_kind  slice
    DW_AT_mutable            true or false // defaults to false if not present
    DW_AT_type               <ref to type>
    DW_AT_object_pointer     <expr that yields address of first element>
    DW_AT_count              <expr that computes count>

// &Struct /w trailing [T]
DW_TAG_reference_type
    DW_AT_rust_pointer_kind  slice
    DW_AT_mutable            true or false // defaults to false if not present
    DW_AT_type               <ref to type>
    DW_AT_object_pointer     <expr that yields address of struct>
    DW_AT_count              <expr that computes count>

// &Trait (regular trait object)
DW_TAG_reference_type
    DW_AT_rust_pointer_kind     trait
    DW_AT_mutable               true or false // defaults to false if not present
    DW_AT_type                  <ref to type>
    DW_AT_object_pointer        <expr that yields address of object>
    DW_AT_vtable_elem_location  <expr that computes address of vtable>

// &Struct /w trailing trait
DW_TAG_reference_type
    DW_AT_rust_pointer_kind     trait
    DW_AT_mutable               true or false // defaults to false if not present
    DW_AT_type                  <ref to type>
    DW_AT_object_pointer        <expr that yields address of object>
    DW_AT_vtable_elem_location  <expr that computes address of vtable>

rust
struct CustomElementData {
    address_ptr: *const (),
    vtable_ptr: *const (),
    type_id: u64
}

#[wasm_bindgen]
fn serialize_in_js(linear_memory_offset: *mut CustomElementData, size: u32) {}

#[wasm_bindgen]
fn deserialize_in_js(linear_memory_offset: *mut CustomElementData) {}

let some_custom_element_data = Box::new(CustomElementData { ... });

let linear_memory_offset = some_custom_element_data.into_raw();
let size = mem::size_of::<CustomElementData>();

serialize_in_js(linear_memory_offset, size as u32);

unsafe {
     mem::drop(Box::from_raw(linear_memory_offset));
}

// Then later...

let uninit_custom_element_data = Box::new(CustomElementData {...});

let linear_memory_offset = uninit_custom_element_data.into_raw();

deserialize_in_js(linear_memory_offset);

// `uninit_custom_element_data` is now initialized with the deserialized data!
let init_custom_element_data = unsafe { Box::from_raw(linear_memory_offset) };

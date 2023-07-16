rust
struct CustomElementData {
    address: *mut (),
    metadata: DynMetadata<dyn Any>,
    type_id: TypeId
}

#[wasm_bindgen]
fn serialize_custom_element_data(
    wasm_memory: &JsValue, 
    offset: *mut CustomElementData, 
    size: u32
) -> js_sys::Uint8Array {}

#[wasm_bindgen]
fn deserialize_custom_element_data(
    wasm_memory: &JsValue, 
    offset: *mut CustomElementData, 
    serialized: &js_sys::Uint8Array
) {}

let mut custom_element_data = CustomElementData { ... };

let offset = &mut custom_element_data as *mut CustomElementData;
let size = mem::size_of::<CustomElementData>();

let serialized = serialize_custom_element_data(&wasm_bindgen::memory(), offset, size as u32);

// Explicitly drop here so it doesn't drop early...
mem::drop(custom_element_data);

// Then later...

let uninit_custom_element_data = MaybeUninit::<CustomElementData>::uninit();
let offset = uninit_custom_element_data.as_mut_ptr();

deserialize_custom_element_data(&wasm_bindgen::memory(), offset, &serialized);

// `uninit_custom_element_data` is now initialized with the deserialized data!
let init_custom_element_data = unsafe { uninit_custom_element_data.assume_init() };

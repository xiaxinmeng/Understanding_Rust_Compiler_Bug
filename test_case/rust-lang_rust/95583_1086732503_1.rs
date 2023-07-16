js
function serialize_in_js(linear_memory_offset, size) {
    // Create a view the relevant region of the WASM linear memory buffer. I'm not currently sure how 
    // one would obtain the reference to the correct WASM linear memory buffer.
    let view = new Uint8Array(wasm_linear_memory, linear_memory_offset, size);

    // Copy it to a new non-view Uint8Array
    let serialized = new Uint8Array(view);

    // Store it somewhere...
}

function deserialize_in_js(linear_memory_offset) {
    // ... retrieve the serialized buffer from wherever it was stored
    let serialized = ...;

    // Overwrite the target region in WASM linear memory buffer with the serialized buffer
    wasm_linear_memory.set(serialized, linear_memory_offset);
}

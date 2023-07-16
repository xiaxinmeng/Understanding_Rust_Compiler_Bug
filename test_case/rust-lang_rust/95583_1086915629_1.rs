js
function serialize_custom_element_data(
    wasm_memory,
    offset,
    size
) {
    // Create a view the relevant region of the WASM linear memory buffer.
    let view = new Uint8Array(wasm_memory.buffer, offset, size);

    // Copy it to a new non-view Uint8Array and return
    return new Uint8Array(view);
}

function deserialize_custom_element_data(
    wasm_memory,
    offset,
    serialized
) {
    // View WASM memory as bytes
    let buffer_view = new Uint8Array(wasm_memory.buffer);

    // Overwrite the memory at the pointer offset with the `serialized` data
    buffer_view.set(serialized, offset);
}

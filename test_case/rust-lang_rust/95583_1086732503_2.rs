rust
// Lets assume there is some constant that represents this "WASM linear memory base pointer"
let my_address_ptr: *const () = WASM_BASE_POINTER.with_addr(my_address_ptr_bits);

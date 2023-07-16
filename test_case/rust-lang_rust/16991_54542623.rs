 rust
type hoedown_realloc_callback = extern fn(*mut c_void, size_t) -> *mut size_t;
type hoedown_free_callback = extern fn(*mut c_void);

 struct hoedown_buffer {
    // ...
    data_realloc: Option<hoedown_realloc_callback>,
    data_free: Option<hoedown_free_callback>,
    buffer_free: Option<hoedown_free_callback>,
 }

 C
int zmq_msg_init_data (
    zmq_msg_t *msg,
    void *data, // Actually a *const ArcInner<T>
    size_t size,
    zmq_free_fn *ffn, // would call unref_arcinner(data), which would be an extern "C" fn written in Rust
    void *hint // Can still use hint to avoid fat pointers
);

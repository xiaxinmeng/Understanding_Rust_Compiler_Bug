rust
static INDEX: AtomicUsize = 0;
static BUFFER: &[u8; however long it needs to be] = /* memory mapped? */;

fn emit(event: Event) {
    let message = event.serialize();
    let my_base = INDEX.fetch_add(message.len(), SeqCst); // Something that also resizes the buffer, if necessary (possibly not an issue for memory mapped files? Make it a ring buffer that something consumes somewhere?)
    BUFFER[my_base..(my_base + message.len)].copy_from_slice(message);    
}

// or:

fn emit(event: Event) {
    let message = event.serialize();
    let my_base = INDEX.fetch_add(message.len(), SeqCst);
    pwrite(logfile, message.ptr(), message.len(), my_base);
}

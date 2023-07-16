rust
pub const unsafe fn rx_buffer_init() -> [BufferDescriptor; BUFFER_CT] {
    transmute::<
        [u8; size_of::<BufferDescriptor>() * BUFFER_CT],
        [BufferDescriptor; BUFFER_CT]
    >([0u8; size_of::<BufferDescriptor>() * BUFFER_CT])
}

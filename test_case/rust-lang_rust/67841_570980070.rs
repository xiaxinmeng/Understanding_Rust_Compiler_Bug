rust
let strategy = if writer.is_write_vectored_optimized() {
    WriteStrategy::Queue
} else {
    WriteStrategy::Flatten
};

rust
for item in extra {
    process_one(item)
}
for chunk in chunkable.exact_chunks(chunk_size) {
    process_many(chunk)
}

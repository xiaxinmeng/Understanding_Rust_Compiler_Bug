rust
static HAS_COPY_FILE_RANGE: AtomicBool = AtomicBool::new(true);

let copy_result = if HAS_COPY_FILE_RANGE.load(Relaxed) {
    copy_file_range(...)
} else {
    ENOSYS
};

if copy_result == ENOSYS {
    HAS_COPY_FILE_RANGE.store(false, Relaxed);
}
if copy_result == ENOSYS || copy_result == EXDEV {
    // attempt simple copy
}

rust
#[cfg(parallel_compiler)]
unsafe impl Sync for FormatArgs {}

#[cfg(parallel_compiler)]
unsafe impl Send for FormatArgs {}

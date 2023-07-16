rust
#![warn(rustdoc::invalid_html_tags)]
#![warn(rustdoc::broken_intra_doc_links)]

pub struct ExistentStruct<T>(T);

/// This [test][ExistentStruct<i32>] thing!
pub struct NoError;

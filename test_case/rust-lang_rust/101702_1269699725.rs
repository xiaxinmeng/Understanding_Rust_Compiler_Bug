rust
pub(crate) struct StaticFile {
    pub(crate) filename: &'static str,
    pub(crate) bytes: &'static [u8],
}

macro_rules! static_files {
    ($($field:ident => $file_path:literal,)+) => {
        pub(crate) struct StaticFiles {
            $($field: StaticFile,)+
        }
        
        pub(crate) const STATIC_FILES: StaticFiles = StaticFiles {
            $($field: StaticFile { filename: $file_path, bytes: include_bytes!($file_path) },)+
        };
        
        pub(crate) static STATIC_FILES_LIST: &[&'static StaticFile] = &[
            $(&STATIC_FILES.$field,)+
        ];
    }
}

// Can be used like this:
static_files!(
    rustdoc_css => "rustdoc.css",
    whatever_css => "whatever.css",
);

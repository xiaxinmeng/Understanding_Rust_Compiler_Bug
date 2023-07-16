rust
pub struct Metadata {
    pub file_type: Option<String>,
    pub file_mime: Option<String>,
    ...
}

fn get_type_and_mime(path: &str) -> Option<(String, String)>;

fn from_path(path: &str) -> Metadata {
    let (file_type, file_mime) = get_type_and_mime(path).map_or( // < this could be replaced by unzip() but you playin
                (None, None),
                |(t0, t1)| (Some(t0), Some(t1))
            );
    // ...
    Metadata {
        file_type,
        file_mime,
        ...
    }
}

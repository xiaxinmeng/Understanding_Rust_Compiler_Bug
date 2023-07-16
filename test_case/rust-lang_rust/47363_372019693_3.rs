rust
    match prefix {
        None | Some(Disk(_)) => {
            if let Some(cleaned) = sys::path::clean_reserved_path(file_name) {
                return cleaned;
            }
        }
        _ => {}
    }
    
Rust
    let mut members: Vec<&mut ArchiveChild<'_>> = vec![];
    let archive_iter = archive.iter();
    loop { match archive_iter.next() {
        Some(child) => {
            members.push(child.raw);
        }
        None => {
            members.len();
            return;
        }
    }}

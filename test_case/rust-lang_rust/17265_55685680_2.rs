
        fn get_err(errno: i32) -> (IoErrorKind, &'static str) {
            // FIXME: this should probably be a bit more descriptive...
            match errno {
                libc::EOF => (EndOfFile, "end of file"),
                <clipped>
            }
        }

        let (kind, desc) = get_err(errno as i32);
        IoError {
            kind: kind,
            desc: desc,
            detail: if detail && kind == OtherIoError {
                Some(os::error_string(errno).as_slice().chars().map(|c| c.to_lowercase()).collect())
            } else {
                None
            },
        }
    }

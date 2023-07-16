 rust
    fn lookup_filemap_idx(&self, pos: BytePos) -> usize {
        let files = self.files.borrow();
        let files = &*files;
        let len = files.len();
        let mut a = 0;
        let mut b = len;
        while b - a > 1 {
            let m = (a + b) / 2;
            if files[m].start_pos > pos {
                b = m;
            } else {
                a = m;
            }
        }
        // There can be filemaps with length 0. These have the same start_pos as
        // the previous filemap, but are not the filemaps we want (because they
        // are length 0, they cannot contain what we are looking for). So,
        // rewind until we find a useful filemap.
        loop {
            let lines = files[a].lines.borrow();
            let lines = lines;
            if !lines.is_empty() {
                break;
            }
            if a == 0 {
                panic!("position {} does not resolve to a source location",
                      pos.to_usize());
            }
            a -= 1;
        }
        if a >= len {
            panic!("position {} does not resolve to a source location",
                  pos.to_usize())
        }

        return a;
    }

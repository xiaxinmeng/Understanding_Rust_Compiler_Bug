 rust
        assert!(SRC_MULT * div <= src_buf.len());
        assert!(DST_MULT * div <= dst_buf.len());
        for i in range(0, div) {
            let x = src_buf.slice(SRC_MULT * i, SRC_MULT * (i + 1));
            let chars = [(x[0] & 0xFC) >> 2,
                         (x[0] & 0x03) << 4 | (x[1] & 0xF0) >> 4,
                         (x[1] & 0x0F) << 2 | (x[2] & 0xC0) >> 6,
                         (x[2] & 0x3F)];
            let y = dst_buf.slice_mut(DST_MULT * i, DST_MULT * (i + 1));
            for i in range(0, chars.len()) {
                y[i] = unsafe { *char_set.unsafe_get(chars[i] as uint) };
            }
        }

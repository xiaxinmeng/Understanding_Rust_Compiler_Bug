 rust
match str::from_utf8_owned_opt(bytes) {
        Some(s) => {
            let s = s.to_managed();
            // Add this input file to the code map to make it available as
            // dependency information
            let mut files = cx.parse_sess.cm.files.borrow_mut();
            files.get().push(@codemap::FileMap {
                name: file.display().to_str().to_managed(),
                substr: codemap::FssNone,
                src: s,
                start_pos: codemap::BytePos(0),  //  <-----  THIS DOESN'T LOOK RIGHT
                lines: RefCell::new(~[]),
                multibyte_chars: RefCell::new(~[]),
            });
            base::MRExpr(cx.expr_str(sp, s))
        }
        None => {
            cx.span_fatal(sp, format!("{} wasn't a utf-8 file", file.display()));
        }
    }

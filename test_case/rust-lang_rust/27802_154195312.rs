 rust
        let mut buf = Cursor::new(&b"\xf0\x9fabc"[..]);
        let mut chars = buf.chars();
        assert!(match chars.next() { Some(Err(CharsError::NotUtf8)) => true, _ => false });
        assert_eq!(chars.next().unwrap().unwrap(), 'a');
        assert_eq!(chars.next().unwrap().unwrap(), 'b');
        assert_eq!(chars.next().unwrap().unwrap(), 'c');
        assert!(chars.next().is_none());

        let mut buf = Cursor::new(&b"\xed\xa0\x80a"[..]);
        let mut chars = buf.chars();
        assert!(match chars.next() { Some(Err(CharsError::NotUtf8)) => true, _ => false });
        assert_eq!(chars.next().unwrap().unwrap(), 'a');
        assert!(chars.next().is_none());

        let mut buf = Cursor::new(&b"\xed\xa0a"[..]);
        let mut chars = buf.chars();
        assert!(match chars.next() { Some(Err(CharsError::NotUtf8)) => true, _ => false });
        assert_eq!(chars.next().unwrap().unwrap(), 'a');
        assert!(chars.next().is_none());

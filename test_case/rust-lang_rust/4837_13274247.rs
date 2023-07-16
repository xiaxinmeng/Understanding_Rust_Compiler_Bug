
pure fn fix_utf8_range(s: &str, begin: uint, end: uint, handler: &fn(&[u8]) -> ~str) -> ~str;
pure fn fix_utf8(s: &str, handler: &fn(&[u8]) -> ~str) -> ~str;

rust
            '\0' => EscapeDebug::backslash(b'\0'.as_ascii().unwrap()),
            '\r' => EscapeDebug::backslash(b'\r'.as_ascii().unwrap()),
            '\n' => EscapeDebug::backslash(b'\n'.as_ascii().unwrap()),

rust
            '\0' => EscapeDebug::backslash(Null),
            '\r' => EscapeDebug::backslash(CarriageReturn),
            '\n' => EscapeDebug::backslash(LineFeed ),

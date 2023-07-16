rust
match 'a' {
    '\u{0}'..='\u{D7FF}' | '\u{E000}'..='\u{10_FFFF}' => {}
}

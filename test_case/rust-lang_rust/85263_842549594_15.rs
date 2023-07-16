rust
match foo.khar {
    '\0'..='\u{D7FF}' | '\u{E000}'..='\u{10FFFF}' => ()
};

rust
union Hmm {
    a: (),
    b: char,
}

fn foo(x: Hmm) -> i32 {
    match x.b {
        '\0'..='\u{D7FF}' | '\u{E000}'..='\u{10FFFF}' => 1,
    }
}

fn bar(x: Hmm) -> i32 {
    match x.b {
        '\0'..='\u{10FFFF}' => 1,
    }
}

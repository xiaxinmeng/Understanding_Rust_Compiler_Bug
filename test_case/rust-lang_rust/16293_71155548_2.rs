 rust
// in libcore:
#[lang="i32"]
enum i32 {} // unihabited enum introduces a type in scope and nothing else

// coherence would allow this impl as the type is "rooted" in the crate
// defining the lang item.
impl i32 {
    fn foo(self) -> ... {...}
}

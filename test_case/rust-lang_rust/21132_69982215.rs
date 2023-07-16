 rust
foo.unwrap() // assertion that "a poison is bad"
foo.unwrap_or_else(|e| e.into_guard()) // assertion that "I'm ok with poisons"

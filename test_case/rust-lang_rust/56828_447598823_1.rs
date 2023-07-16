rust
match (Thing {})
{} // Parsed as the match block.  ERROR: Missing `_` branch


let thing: ! = panic!();

match thing {}
{} // Parsed as a block expression statement

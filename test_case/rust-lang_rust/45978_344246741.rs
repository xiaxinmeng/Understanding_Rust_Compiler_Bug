rust
if file.is_none() { return; }
let Some(f) = file;
// Unacceptable, the signature `fn is_none(&self) -> bool` tells nothing 
// about the typestate of `file`

rs
let func = |should_not_be_mutable: &mut H<u8>| { };

breaks(func); // Oops! ICE!

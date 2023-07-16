rust
asm!("", options()); // <- warning: useless options
asm!("", options(att_syntax, nomem), options(att_syntax));
//                                   ^--- warning: useless options

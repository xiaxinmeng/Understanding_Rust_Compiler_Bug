 rust
LICENSE

extern crate foo as bar;
//~^ ERROR expected `;`, found `as`; perhaps you meant to enclose the crate name (`foo`) in a string?

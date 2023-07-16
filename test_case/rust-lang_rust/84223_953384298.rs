rust
#![feature(inline_const)]
#![feature(thread_local_const_init)]

use std::cell::Cell;

static V: u32 = 1;

std::thread_local!{
	// This is accepted, because this isn't actually const.
    static KEY: Cell<u32> = const { Cell::new(V) };

	// This is not, because this is actually const, and const cannot refer to statics.
    static KEY2: Cell<u32> = (const { Cell::new(V) });
}

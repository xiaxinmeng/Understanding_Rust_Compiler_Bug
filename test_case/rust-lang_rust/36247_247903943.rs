
// GPIOD is an extern struct so that I can place it on memory-mapped
// registers and keep Rust from trying to initialize it.  Its only public
// operations are set_pin and clear_pin, both of which use unsafe code
// but are intended as safe APIs.  But safe code cannot do this:
stm32f4::gpio::GPIOD.set_pin(12);

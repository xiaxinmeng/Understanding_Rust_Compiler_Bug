

Building stage2 tool error_index_generator (x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.0 secs
Testing error-index stage2

running 563 tests
..ii...................................i....................i.........ii.Fi...........i.i..........i.........................i.ii...........ii........i......i.....ii.ii.......i..i..............................i........................ii..................................i..........................i.i.........i.i.....i........i.............i.........i.........i..ii............................i..i.i.i............ii...i...i.i.i.i.i........................................i.......i..............................................i.i...............i.i................
failures:

---- Rust_Compiler_Error_Index_167 stdout ----
	error: language item required, but not found: `panic_fmt`

error: language item required, but not found: `eh_personality`

error: aborting due to previous error(s)

thread 'Rust_Compiler_Error_Index_167' panicked at 'Box<Any>', /checkout/src/librustc/session/mod.rs:201


failures:
    Rust_Compiler_Error_Index_167

test result: FAILED. 506 passed; 1 failed; 56 ignored; 0 measured

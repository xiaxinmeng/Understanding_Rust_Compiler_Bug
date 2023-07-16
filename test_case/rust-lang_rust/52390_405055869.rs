
(lldb) run
Process 46736 launched: './diceware-c7f6b180dd3b52c6' (x86_64)

running 3 tests
Process 46736 stopped
* thread #2: tid = 0x959310, 0x00000001000cc9d2 diceware-c7f6b180dd3b52c6`std::panicking::panicking::h01f4a398b1d2259a + 66 at ptr.rs:237, name = 'tests::returns_an_error_if_number_of_words_is_zero', stop reason = EXC_BAD_ACCESS (code=EXC_I386_GPFLT)
    frame #0: 0x00000001000cc9d2 diceware-c7f6b180dd3b52c6`std::panicking::panicking::h01f4a398b1d2259a + 66 at ptr.rs:237

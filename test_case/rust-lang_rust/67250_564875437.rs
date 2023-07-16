
  │     Total:     107,143,656 bytes (5.6%, 9,030.59/Minstr) in 787,821 blocks (6.57%, 66.4/Minstr), avg size 136 bytes, avg lifetime 0 instrs (0% of program duration)
  │     At t-gmax: 107,143,656 bytes (5.6%) in 787,821 blocks (6.57%), avg size 136 bytes
  │     At t-end:  107,143,656 bytes (5.6%) in 787,821 blocks (6.57%), avg size 136 bytes
  │     Reads:     0 bytes (0%, 0/Minstr), 0/byte
  │     Writes:    0 bytes (0%, 0/Minstr), 0/byte
  │     Allocated at {
  │       ^1: 0x483B247: memmove (dh_replace_memcpy.c:227)
  │       ^2: 0x5A5CE52: clone<[syntax_expand::mbe::macro_parser::NamedMatch; 4]> (lib.rs:1454)
  │       ^3: 0x5A5CE52: alloc::rc::Rc<T>::make_mut (rc.rs:857)
  │       #4: 0x5ADB4F6: syntax_expand::mbe::macro_parser::MatcherPos::push_match (macro_parser.rs:223)
  │     }


 Total:     798,080 bytes (0.54%, 544.46/Minstr) in 24,728 blocks (2.68%, 16.87/Minstr), avg size 32.27 bytes, avg lifetime 60,211.12 instrs (0% of program duration)
 Max:       7,008 bytes in 219 blocks, avg size 32 bytes
 At t-gmax: 0 bytes (0%) in 0 blocks (0%), avg size 0 bytes
 At t-end:  0 bytes (0%) in 0 blocks (0%), avg size 0 bytes
 Reads:     852,604 bytes (0.16%, 581.66/Minstr), 1.07/byte
 Writes:    801,120 bytes (0.42%, 546.54/Minstr), 1/byte
 Allocated at {
   ^1: 0x51EEA4B: alloc (alloc.rs:84)
   ^2: 0x51EEA4B: alloc (alloc.rs:172)
   ^3: 0x51EEA4B: reserve_internal<(syntax_pos::span_encoding::Span, alloc::string::String),alloc::alloc::Global> (raw_vec.rs:695)
   ^4: 0x51EEA4B: reserve<(syntax_pos::span_encoding::Span, alloc::string::String),alloc::alloc::Global> (raw_vec.rs:520)
   ^5: 0x51EEA4B: alloc::vec::Vec<T>::reserve (vec.rs:501)
   #6: 0x545C9E3: push<rustc_mir::build::matches::MatchPair> (vec.rs:1145)
   #7: 0x545C9E3: rustc_mir::build::matches::simplify::<impl rustc_mir::build::Builder>::simplify_candidate (simplify.rs:39)
   #8: 0x5464F6B: rustc_mir::build::matches::<impl rustc_mir::build::Builder>::match_candidates (mod.rs:826)
 }
 
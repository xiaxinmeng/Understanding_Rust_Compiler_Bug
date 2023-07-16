
src/ostinato/music.rs:101:46: 101:57 error: internal compiler error: aliasability violation with closure
src/ostinato/music.rs:101         self.map(&mut|&: e| e.map_pitch(&mut |&: p| f(p)))
                                                                       ^~~~~~~~~~~
note: the compiler unexpectedly panicked. this is a bug.

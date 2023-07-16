
 lilian  ~  rustc +stable --crate-type staticlib /tmp/conftestVB4hBn.rs                                                            1:18:16
note: link against the following native artifacts when linking against this static library

note: the order and any duplication can be significant on some platforms, and so may need to be preserved

note: library: dl

note: library: rt

note: library: pthread

note: library: gcc_s

note: library: c

note: library: m

note: library: rt

note: library: util

 lilian  ~  rustc +nightly --crate-type staticlib /tmp/conftestVB4hBn.rs                                                           1:18:30
note: link against the following native artifacts when linking against this static library

note: the order and any duplication can be significant on some platforms, and so may need to be preserved

note: library: dl

note: library: rt

note: library: pthread

note: library: gcc_s

note: library: c

note: library: m

note: library: rt

note: library: pthread

note: library: util

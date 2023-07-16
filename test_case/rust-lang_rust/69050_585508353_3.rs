
IMPROVEMENTS
            clap-rs-Check-CleanIncr
feb10/Leb0  8,992M        $RUSTC0
feb10/Leb1  8,927M/99.3%  First attempt
feb11/Leb4  8,996M        $RUSTC0 but with bounds checking
feb11/Leb5  8,983M        `loop` for reading
feb11/Leb6  8,928M/99.3%  `loop` for writing, `write_to_vec` removed
feb11/Leb8  8,829M/98.1%  avoid mask on final byte in read loop
feb11/Leb9  8,529M/94.8%  in write loop, avoid a condition
feb11/Leb10 8,488M/94.4%  in write loop, mask/shift on final byte
feb13/Leb13 8,488M/94.4%  in write loop, push `(value | 0x80) as u8`
feb13/Leb15 8,488M/94.4%  in read loop, do `as` before `&`
feb13/Leb18 8,492M/94.4%  Landed (not sure about the extra 4M, oh well)

REGRESSIONS
feb11/Leb2  8,927M/99.3%  add slice0, slice1, slice2 vars
feb11/Leb3  9,127M        move the slow loop into a separate no-inline function
feb11/Leb7  8,930M        `< 128` in read loop
feb11/Leb11 8,492M        use `byte < 0x80` in read loop
feb12/Leb12 8,721M        unsafe pushing in write
feb13/Leb14 8,494M/94.4%  in write loop, push `(value as u8) | 0x80`
feb13/Leb16 8,831M        eddyb's write loop
feb13/Leb17 8,578M        eddyb's read loop


cnt = 0u32 // Honestly, if this is a register why would it ever be faster? Is LLVM spitting out bad 64bit code?
while( cnt & ~((usize::MAX - 1) >> 1) != 0) { // Time for check is amortized out
    if( iter.next() is not None ) {
        cnt++
    } else {break}
    ... // Number of copy pastes chosen empirically
        // Best if it changes # of iterations based on the ISA
    if( iter.next() is not None ) {
        cnt++
    } else {break}
}
// The above code should be skipped during compilation if its to a 16bit or lower ISA
fullcnt = cnt as u64
while( iter.next() is not None ) {
    fullcnt++
}

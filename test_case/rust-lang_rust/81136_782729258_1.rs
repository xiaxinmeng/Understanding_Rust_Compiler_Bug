rust
struct XorState {
    sum: u8 =0;
}

fun xor_fold( &mut state: XorState, buf: &mut Vec<u8>, byte: u8){
   // while (buf.peek() != byte)
   // state.sum = xor(state.sum, buf.get() );
}



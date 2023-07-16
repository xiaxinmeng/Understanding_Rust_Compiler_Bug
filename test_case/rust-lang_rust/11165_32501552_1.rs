 rust
let pending_op= selectableStream.read_some_async(
    ByteBuffer { buffer: ~[0, ..100], offset: 0, length: 20}
).unwrap();
let result = selector.wait().unwrap();

if (result.pending_op == pending_op) {
    let read_result = pending_op.end_read_async();
}

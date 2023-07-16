rust
let mut fixed_len_channel = channel(1); // fixed-len channel of length 1
start_threads();
for item in work_items {
    let llvm_ir = translate(next_item);
    fixed_len_channel.send(llvm_ir); // will block if channel is full
}
free_tcx_etc();
join_threads();

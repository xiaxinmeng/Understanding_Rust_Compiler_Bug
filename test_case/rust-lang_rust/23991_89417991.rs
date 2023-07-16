
let remaining_capacity = buf.capacity() - buf.len();
buf.extend(iter::repeat(0).take(cmp::min(new_write_size, remaining_capacity)));

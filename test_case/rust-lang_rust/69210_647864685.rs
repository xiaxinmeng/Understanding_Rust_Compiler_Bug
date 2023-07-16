rust
let split_list = original_list.split_off(index_to_remove);
split_list.pop_front();
original_list.append(split_list);

rust
if a.len() == 2 {
    do_work_on_two_array(*a.try_into().unwrap());
}

fn do_work_on_two_array<T>(_: [T; 2]){}

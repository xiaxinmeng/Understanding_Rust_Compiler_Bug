
drop_selector:
    ptr = &mut LV[0];
    index = 0;
    len = len(LV);
    is_zero_sized = size_of::<T>() == 0;
    if is_zero_sized then zs_head else loop_head;
zs_head:
    finished = index == len;
    if finished then succ else zs_body;
zs_body:
    drop(ptr);
    index = index + 1;
    goto zs_head;
loop_head:
    finished = ptr == end;
    if finished then succ else loop_body
loop_body:
    drop(ptr);
    ptr = ptr + size_of::<T>();
    goto loop_head;

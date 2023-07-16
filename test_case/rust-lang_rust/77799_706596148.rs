
error: Undefined Behavior: pointer to alloc17796 was dereferenced after this allocation got freed
   --> src/main.rs:89:23
    |
89  |                 cur = *next;
    |                       ^^^^^ pointer to alloc17796 was dereferenced after this allocation got freed

rust
// 'u and 'v are infernce variables, not late-bound
let controller: <O as AsWorkspaceController<'u, 'v>>::Controller = ...;
let temp = &'w controller;
let log = <<O as AsWorkspaceController<'u, 'v>>::Controller as WorkspaceController<'x>>::get_log(temp);

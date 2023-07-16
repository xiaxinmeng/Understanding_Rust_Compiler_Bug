 rust
struct StateLayout { L: *mut raw::lua_State, stackspace: i32 }

pub struct State { data: StateLayout }

pub struct ExternState<'a> { data: StateLayout }
pub struct RawState<'a> { data: StateLayout }

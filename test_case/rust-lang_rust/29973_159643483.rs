
#[cfg(target_env = "msvc")] type __enum_ty = i32;
#[cfg(not(target_env = "msvc"))] type __enum_ty = u32;
// then in some macro
pub type $name = __enum_ty;


steve@warmachine:~/tmp/foo$ cat src/lib.rs 
pub fn elided(r: &i32) -> &i32 { r }
pub fn not<'a>(r: &'a i32) -> &'a i32 { r }

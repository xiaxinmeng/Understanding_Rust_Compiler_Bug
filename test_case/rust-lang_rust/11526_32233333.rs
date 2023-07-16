 rust
#[repr(u8)]
pub enum bool {
    true = 1,
    false = 0,
}

#[lang="u8"] pub struct u8;
#[lang="u16"] pub struct u16;
#[lang="u32"] pub struct u32;
#[lang="u64"] pub struct u64;
#[lang="uintptr"] pub struct uintptr;

#[lang="i8"] pub struct i8;
#[lang="i16"] pub struct i16;
#[lang="i32"] pub struct i32;
#[lang="i64"] pub struct i64;
#[lang="intptr"] pub struct intptr;

#[lang="f32"] pub struct f32;
#[lang="f64"] pub struct f64;

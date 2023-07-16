rust
>macro define_and_export() {
>    pub const hello: u32 = 0;
>}
>
>define_and_export!();
>pub const hello: u32 = 1;
>
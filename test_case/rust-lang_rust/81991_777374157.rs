rust
async fn async_dummy() {}

async fn async_extra_semicolon_same() {
    let _ = match true {
        true => {
            async_dummy();
        }
        false => async_dummy(),
    };
}

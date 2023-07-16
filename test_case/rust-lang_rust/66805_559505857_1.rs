rust
#[macro_export]
macro_rules! stream {
    ($($body:tt)*) => {{
        crash! {
             {
                 $($body)*
             }
        }
        stream_0!()
    }}  
}

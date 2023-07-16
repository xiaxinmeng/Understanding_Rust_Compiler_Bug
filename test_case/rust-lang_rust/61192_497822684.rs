rust
async fn response(
    data: Vec<u8> // This is an internal variable, it should have its Span marked
) -> ::std::future::from_generator(move || {
    let data // This variable is effectively the user's parameter, it should not have it's Span marked
        = data; 
    data.reverse();
}) 

rust
// imaginary generator syntax stolen from JavaScript
fn* my_generator() -> T {
    yield some_value;

    // explicit return statements are only included to 
    // make it clear the generator/async functions are finished.
    return another_value;
}

// `await` keyword would not be allowed here, but the `yield` keyword is
#[async]
fn* my_async_generator() -> Result<T, E> {
    let item = some_op().await!()?; // uses the `.await!()` macro
    // which would really just use `yield` internally, but with the pinning API

    yield future::ok(item.clone());

    return Ok(item);
}

// `yield` would not be allowed here, but the `await` keyword is.
async fn regular_async() -> Result<T, E> {
   let some_op = async || { /*...*/ };

   let item = some_op() await?;

   return Ok(item);
}

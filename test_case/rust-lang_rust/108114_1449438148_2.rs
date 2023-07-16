rust
use core::future::{Future};
use anyhow::Error;

struct Session {}

impl Session {
    pub async fn dispatch_request<T, F, Fut>(&self, interpreter: F) -> Result<T, Error>
        where
        F: for<'b> FnMut(&'b mut String) -> Fut,
        Fut: Future<Output = Result<T, Error>>,
    {
        let mut interpreter = interpreter;
        let mut stream = "hello".to_string();
        interpreter(&mut stream).await
    }
}

async fn test_interpreter(input_stream: &mut String) -> Result<String, Error> {

    Ok(input_stream.clone())
}

#[tokio::main]
async fn main() {
    let session = Session{};

    session.dispatch_request(test_interpreter).await.unwrap();
}


#[async_std::main]
async fn main() {
  task::spawn(async {
    loop {
      other::crate::async_fn(&"test_string".to_string()).await;
    }
  });
}

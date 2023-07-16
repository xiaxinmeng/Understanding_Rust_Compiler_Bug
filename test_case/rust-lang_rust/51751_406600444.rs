
#![feature(async_await, await_macro, futures_api)]

use std::task::Executor;

async fn inc(limit: i64) -> i64 {
    limit + 1
}

async fn run() {
  let result = await!(inc(10000));

  println!("{}", result);
}

fn main() {
  Executor::spawn_obj(&self, run);
}

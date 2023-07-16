rust
use std::ops::ControlFlow;

pub fn example_try_fold(m: i32, max: i32) -> i32 {
  match (1..=max).try_fold(0, |acc, i: i32| {
      if i * i == m { 
          ControlFlow::Break(i)
      }
      else {
        ControlFlow::Continue(acc)
      }
  }) {
      ControlFlow::Break(i) | ControlFlow::Continue(i) => i,
  }
}

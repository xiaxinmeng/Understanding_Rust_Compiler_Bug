rust
//   Cacher.rs

use std::marker::PhantomData;
use std::mem;
use std::thread;
use std::time::Duration;

struct Cacher<T, U, R>
    where
        T: Fn(U) -> R
{
    calculation: T,
    value: Option<R>,
    __phantom: PhantomData<U>,
}

impl<T, U, R> Cacher<T, U, R>
    where
        T: Fn(U) -> R
{
    fn new(calculation: T) -> Self {
        Self {
            calculation,
            value: None,
            __phantom: PhantomData,
        }
    }

    fn value(&mut self, arg: U) -> &R {
        let current = &mut self.value;
        match current {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                mem::replace(current, Some(v));
                let current = current.as_ref();
                current.unwrap()
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut expensive_result = Cacher::new(|input: i32| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            input * 2
        });

        println!(
            "Today, do {} pushups!",
            expensive_result.value(1)
        );

        println!(
            "Next, do {} situps!",
            expensive_result.value(1)
        );

        let correctOutput = &2;
        assert_eq!(correctOutput, expensive_result.value(1));
    }
}


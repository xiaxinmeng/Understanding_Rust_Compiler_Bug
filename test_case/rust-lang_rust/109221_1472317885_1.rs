rust
pub struct EventKey<I, C> {
    invoker: I,
    callbacks: Vec<C>,
}

impl<I, C> EventKey<I, C> {
    pub fn new(invoker: I) -> EventKey<I, C> {
        EventKey {
            invoker,
            callbacks: Vec::new(),
        }
    }

    pub fn on_event(&mut self, callback: C) {
        self.callbacks.push(callback);
    }

    pub fn invoke<'a, R>(&'a self) -> R
    where
        I: Fn(&'a [C]) -> R + 'a,
        R: 'a,
        C: 'a,
    {
        (self.invoker)(&self.callbacks)
    }
}

#[cfg(test)]
mod tests {
    use super::EventKey;
    fn test(mut x: i32, y: i32, entries: &[&dyn Fn(i32)]) -> i32 {
        for entry in entries {
            x *= 3;
            (entry)(x * y);
        }
        x * y
    }
    fn invoker<'a>(entries: &'a [&'a dyn Fn(i32)]) -> impl Fn(i32) -> i32 + 'a {
        move |y| test(1, y, entries)
    }
    fn func(x: i32) {
        println!("{}", x);
    }
    #[test]
    fn it_works() {
        let mut event_test = EventKey::new(invoker);
        event_test.on_event(&func);
        let result = event_test.invoke()(10);
        assert_eq!(result, 30);
    }
}

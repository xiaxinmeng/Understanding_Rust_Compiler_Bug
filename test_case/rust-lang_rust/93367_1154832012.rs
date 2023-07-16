rust
struct CustomSend<T: SomeTrait, U>(T, U);

unsafe impl<U> CustomSend<NotSendType, U> {}

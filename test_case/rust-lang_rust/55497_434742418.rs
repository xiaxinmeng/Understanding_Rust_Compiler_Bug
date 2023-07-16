rust
>>fn hylo<F0, F1, S, T0, T1>(f: F0, g: F1, x: T1) -> T0
>>where
>>    F0: Fn(ListF<S, T0>) -> T0 + Copy,
>>    F1: Fn(T1) -> ListF<S, T1> + Copy,
>>{
>>    f(map(|a| hylo(f, g, a), g(x)))
>>}
>>
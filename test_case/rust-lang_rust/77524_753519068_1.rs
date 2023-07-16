
error[E0107]: missing generics for struct `JoinHandle`
    --> main.rs:6:18
     |
6    |     handle1: Arc<JoinHandle>,
     |                  ^^^^^^^^^^ expected 1 type argument
     |
note: struct defined here, with 1 type parameter: `T`
    --> /home/ppp/Projects/rust/library/std/src/thread/mod.rs:1312:12
     |
1312 | pub struct JoinHandle<T>(JoinInner<T>);
     |            ^^^^^^^^^^ -
help: use angle brackets to add missing type argument
     |
6    |     handle1: Arc<JoinHandle<T>>,
     |                            ^^^

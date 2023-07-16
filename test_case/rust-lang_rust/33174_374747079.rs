
error[E0446]: private type `Inner` in public interface
 --> src/main.rs:6:1
  |
6 | pub type Doop = DoopGeneral<Inner>;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't leak private type

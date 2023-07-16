
error: use of deprecated item 'core::str::<impl str>::trim_right': superseded by `trim_end`
  --> src\libstd\sys\windows\os.rs:79:31
   |
79 |                 let len = msg.trim_right().len();
   |                               ^^^^^^^^^^
   |
   = note: `-D deprecated` implied by `-D warnings`

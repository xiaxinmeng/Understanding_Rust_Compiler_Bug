
error[E0428]: the name `frida_init` is defined multiple times
 --> src/lib.rs:6:1
  |
3 |     fn frida_init();
  |     ---------------- previous definition of the value `frida_init` here
...
6 | pub extern "C" fn frida_init(){
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `frida_init` redefined here
  |
  = note: `frida_init` must be defined only once in the value namespace of this module

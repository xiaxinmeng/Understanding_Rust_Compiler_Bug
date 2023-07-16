plain
   Compiling hashbrown v0.12.0
   Compiling object v0.26.2
   Compiling miniz_oxide v0.4.0
   Compiling addr2line v0.16.0
error[E0412]: cannot find type `off64_t` in this scope
   --> library/std/src/sys/unix/fd.rs:112:27
112 |                 offset as off64_t,
    |                           ^^^^^^^ not found in this scope
    |
help: consider importing this type alias
help: consider importing this type alias
    |
6   | use libc::off64_t;
    |

error[E0412]: cannot find type `off64_t` in this scope
   --> library/std/src/sys/unix/fd.rs:179:27
179 |                 offset as off64_t,
    |                           ^^^^^^^ not found in this scope
    |
help: consider importing this type alias

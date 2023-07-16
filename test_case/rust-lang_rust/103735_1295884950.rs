plain
   Compiling cpufeatures v0.2.1
   Compiling serde v1.0.147
   Compiling rustc_arena v0.0.0 (/checkout/compiler/rustc_arena)
   Compiling unic-char-range v0.9.0
error: implicit auto-ref creates a reference to a dereference of a raw pointer
   --> compiler/rustc_arena/src/lib.rs:106:34
    |
106 |                 self.start().add((*self.storage.as_ptr()).len())
    |
    |
    = note: creating a reference inflicts a lot of safety requirements
    = note: `#[deny(implicit_unsafe_autorefs)]` on by default
help: if this reference is intentional, make it explicit
    |
106 |                 self.start().add((&(*self.storage.as_ptr())).len())
    |                                  ++                        +

error: implicit auto-ref creates a reference to a dereference of a raw pointer
   --> compiler/rustc_arena/src/lib.rs:290:27
    |
290 |                 new_cap = (*last_chunk.storage.as_ptr()).len().min(HUGE_PAGE / elem_size / 2);
    |
    |
    = note: creating a reference inflicts a lot of safety requirements
help: if this reference is intentional, make it explicit
    |
290 |                 new_cap = (&(*last_chunk.storage.as_ptr())).len().min(HUGE_PAGE / elem_size / 2);
    |                           ++                              +

error: implicit auto-ref creates a reference to a dereference of a raw pointer
   --> compiler/rustc_arena/src/lib.rs:398:27
    |
398 |                 new_cap = (*last_chunk.storage.as_ptr()).len().min(HUGE_PAGE / 2);
    |
    |
    = note: creating a reference inflicts a lot of safety requirements
help: if this reference is intentional, make it explicit
    |
398 |                 new_cap = (&(*last_chunk.storage.as_ptr())).len().min(HUGE_PAGE / 2);
    |                           ++                              +
error: could not compile `rustc_arena` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:04:35
